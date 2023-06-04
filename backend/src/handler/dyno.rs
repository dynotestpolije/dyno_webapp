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
    actions::dyno as dyno_actions, actions::info as info_actions, handler::DynoUrlsQueries,
    middlewares::JwtUserMiddleware, models::dyno::NewDynos,
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
    dbpool: web::Data<crate::DynoDBPool>,
    cfg: web::Data<crate::config::ServerConfig>,
) -> DynoResult<HttpResponse> {
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
        let dyno_config: DynoTestDataInfo =
            DynoTestDataInfo::decompress(&info_stream).map_err(|err| {
                DynoErr::bad_request_error(format!(
                    "Multipart POST data is invalid, there no json config - {err}",
                ))
            })?;

        let checksum = checksum_from_bytes(&data_stream);
        if compare_checksums(checksum.as_bytes(), dyno_config.checksum_hex.as_bytes()) {
            let config = dyno_config.config.clone();
            let info_id = info_actions::insert(&mut conn, config.into()).ok();

            let new_dyno = NewDynos::new(info_id, id, dyno_config);
            let dyno_id = dyno_actions::insert(&mut conn, new_dyno)?;

            let file_path = root_path.join(format!("/data/dyno/{dyno_id}-{id}-{uuid}",));
            std::fs::write(file_path, data_stream).map_err(DynoErr::internal_server_error)
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
    dbpool: web::Data<crate::DynoDBPool>,
) -> DynoResult<HttpResponse> {
    let user_id = user_id.unwrap_or(session.id);

    let blk_result = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| match id {
                Some(id) => dyno_actions::select(&mut conn, id, user_id).map(OneOrMany::One),
                None => dyno_actions::select_many(&mut conn, user_id, max).map(OneOrMany::Many),
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    blk_result.map(|x| match x {
        OneOrMany::One(one) => HttpResponse::Ok().json(ApiResponse::success(one)),
        OneOrMany::Many(many) => HttpResponse::Ok().json(ApiResponse::success(many)),
    })
}

#[get("/dyno/file/{id}")]
pub async fn get_dyno_file_path(
    id: web::Path<i32>,
    JwtUserMiddleware(session): JwtUserMiddleware,
    dbpool: web::Data<crate::DynoDBPool>,
) -> DynoResult<HttpResponse> {
    let user_id = session.id;
    let blk_result = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| dyno_actions::select_id(&mut conn, id.into_inner(), user_id))
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    blk_result.map(|dyno_id| {
        let id = session.id;
        let uuid = session.uuid;
        HttpResponse::Ok().json(ApiResponse::success(format!(
            "/data/dyno/{dyno_id}-{id}-{uuid}"
        )))
    })
}
