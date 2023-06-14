use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse};
use dyno_core::{
    crypto::{checksum_from_bytes, compare_checksums},
    dynotests::DynoTestDataInfo,
    users::OneOrMany,
    ApiResponse, CompresedSaver, DynoErr, DynoResult,
};
use futures::TryStreamExt;
// use futures::TryStreamExt;

use crate::{
    actions::dyno as dyno_actions,
    actions::info as info_actions,
    handler::DynoUrlsQueries,
    middlewares::JwtUserMiddleware,
    models::dyno::{Dynos, NewDynos},
};

/// # Dynotest Endpoint `add_dyno`
/// -----------------------------------------------------------------
/// URL                 => `/api/dyno`
/// GUARD               => `POST`
/// HEADER/COOKIES      => [`crate::middlewares::JwtUserMiddleware`]
/// BODY(JSON)          => 'dyno_core::model::dynotests::DynoTestDataInfo '
///
/// -----------------------------------------------------------------
#[post("/dyno")]
pub async fn add_dyno(
    mut payload: Multipart,
    JwtUserMiddleware(session): JwtUserMiddleware,
    data: web::Data<crate::ServerState>,
) -> DynoResult<HttpResponse> {
    let dbpool = data.db.clone();
    let cfg = &data.cfg;
    let mut info_stream = web::BytesMut::with_capacity(core::mem::size_of::<DynoTestDataInfo>());
    let mut data_stream = web::BytesMut::new();

    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(DynoErr::internal_server_error)?
    {
        let content_type = field.content_disposition();
        let Some(field_name) = content_type.get_name() else {
            continue;
        };
        if field_name == "data" {
            while let Some(chunk) = field
                .try_next()
                .await
                .map_err(DynoErr::internal_server_error)?
            {
                data_stream.extend_from_slice(&chunk)
            }
        } else if field_name == "info" {
            while let Some(chunk) = field
                .try_next()
                .await
                .map_err(DynoErr::internal_server_error)?
            {
                info_stream.extend_from_slice(&chunk)
            }
        }
    }
    let id = session.id;
    let uuid = session.uuid;
    let root_path = cfg.app_root_path.clone();
    let blk_result = web::block(move || {
        let mut conn = dbpool.get().map_err(DynoErr::database_error)?;
        let dyno_config = DynoTestDataInfo::decompress(&info_stream).map_err(|err| {
            DynoErr::bad_request_error(format!(
                "Multipart POST data is invalid, there no json config - {err}",
            ))
        })?;

        let checksum = checksum_from_bytes(&data_stream);
        if compare_checksums(checksum.as_bytes(), dyno_config.checksum_hex.as_bytes()) {
            let config = dyno_config.config.clone();
            let info_id = info_actions::insert(&mut conn, config.into()).ok();

            let new_dyno = NewDynos::new(id, info_id, dyno_config);
            let dyno_id = dyno_actions::insert(&mut conn, new_dyno)?;

            std::fs::write(
                root_path.join(format!("/data/dyno/{dyno_id}-{id}-{uuid}")),
                data_stream,
            )
            .map_err(DynoErr::internal_server_error)
        } else {
            Err(DynoErr::expectation_failed_error(
                "Failed receive data, checksum of data stream is not the same",
            ))
        }
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    blk_result.map(|_| HttpResponse::Ok().finish())
}

/// # Dynotest Endpoint `get_dyno`
/// -----------------------------------------------------------------
/// URL                 => `/api/dyno/{query}` [query = `DynoUrlsQueries`]
/// GUARD               => `GET`
/// HEADER/COOKIES      => [`crate::middlewares::JwtUserMiddleware`]
/// BODY(JSON)          => ['crate::dyno_core::model::dynotests::DynoTestDataInfo']
///
/// -----------------------------------------------------------------
#[get("/dyno")]
pub async fn get_dyno(
    web::Query(DynoUrlsQueries { id, user_id, max }): web::Query<DynoUrlsQueries>,
    JwtUserMiddleware(session): JwtUserMiddleware,
    data: web::Data<crate::ServerState>,
) -> DynoResult<HttpResponse> {
    let dbpool = data.db.clone();

    let user_id = user_id.unwrap_or(session.id);
    let blk_result = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| match id {
                Some(id) => dyno_actions::select(&mut conn, id, user_id)
                    .map(|x| OneOrMany::One(Dynos::into_response(x))),
                None => dyno_actions::select_many(&mut conn, user_id, max).map(|x| {
                    OneOrMany::Many(x.into_iter().map(Dynos::into_response).collect::<Vec<_>>())
                }),
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    blk_result.map(|x| match x {
        OneOrMany::One(one) => HttpResponse::Ok().json(ApiResponse::success(one)),
        OneOrMany::Many(many) => HttpResponse::Ok().json(ApiResponse::success(many)),
    })
}
