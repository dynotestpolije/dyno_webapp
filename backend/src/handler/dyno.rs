use std::path::Path as StdPath;

use actix_web::{
    get,
    http::header,
    post,
    web::{self, Path},
    HttpResponse,
};

use actix_multipart::Multipart;
use dyno_core::{
    crypto::{checksum_from_bytes, compare_checksums},
    dynotests::DynoTestDataInfo,
    users::OneOrMany,
    ApiResponse, BufferData, CompresedSaver, CsvSaver, DynoErr, DynoResult, ExcelSaver,
};
use futures::TryStreamExt;

use crate::{
    actions::dyno as dyno_actions,
    actions::info as info_actions,
    handler::DynoUrlsQueries,
    middlewares::JwtUserMiddleware,
    models::{
        dyno::{Dynos, NewDynos},
        uuid::UUID,
    },
};

#[inline]
fn save_dyno(
    path: impl AsRef<StdPath>,
    data: impl AsRef<[u8]>,
    id: i64,
    uuid: impl std::fmt::Display + Copy,
) -> DynoResult<()> {
    let path = path.as_ref();
    dyno_core::log::debug!("Save dyno in {}, with id:{id} uuid:{uuid}", path.display());
    if !path.exists() {
        if let Err(err) = std::fs::create_dir_all(path) {
            return Err(DynoErr::internal_server_error(err));
        }
    }
    std::fs::write(path.join(format!("{id}-{uuid}.dyno")), data)
        .map_err(DynoErr::internal_server_error)
}

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
    let public_path = cfg.app_public_path.clone();
    let blk_result = web::block(move || {
        let mut conn = dbpool
            .get()
            .map_err(|_| DynoErr::database_error("Failed to get database connection"))?;
        let dyno_config = DynoTestDataInfo::decompress(&info_stream).map_err(|err| {
            DynoErr::bad_request_error(format!("Multipart POST 'info' part is invalid - {err}",))
        })?;

        let checksum = checksum_from_bytes(&data_stream);
        if !compare_checksums(checksum.as_bytes(), dyno_config.checksum_hex.as_bytes()) {
            return Err(DynoErr::expectation_failed_error(
                "Failed receive data, checksum of 'data' part stream is not the same",
            ));
        }

        let info_id = info_actions::insert(&mut conn, dyno_config.config.clone().into()).ok();

        let last_dyno_id = dyno_actions::get_last_id(&mut conn)?;
        let user_path = format!("dyno/{uuid}");
        let dyno_uuid = UUID::new();
        save_dyno(
            public_path.join(&user_path),
            data_stream,
            last_dyno_id + 1,
            dyno_uuid,
        )
        .and_then(|_| {
            dyno_actions::insert(
                &mut conn,
                NewDynos {
                    user_id: id,
                    info_id,
                    uuid: dyno_uuid,
                    data_url: format!("/{user_path}/{}-{dyno_uuid}.dyno", last_dyno_id + 1),
                    data_checksum: checksum,
                    start: dyno_config.start,
                    stop: dyno_config.stop,
                },
            )
        })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    blk_result.map(|id| HttpResponse::Ok().json(ApiResponse::success(id)))
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
    web::Query(DynoUrlsQueries {
        id,
        max,
        all,
        admin,
    }): web::Query<DynoUrlsQueries>,
    JwtUserMiddleware(session): JwtUserMiddleware,
    data: web::Data<crate::ServerState>,
) -> DynoResult<HttpResponse> {
    let dbpool = data.db.clone();

    let is_admin = session.role.is_admin();
    let admin_query = admin.is_some_and(|x| x);
    if admin_query && !is_admin {
        return Err(DynoErr::unauthorized_error(
            "NotAuthorized! Admin Access required!",
        ));
    }

    let user_id = session.id;
    let blk_result = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| match id {
                Some(id) => dyno_actions::select_by_id(&mut conn, id)
                    .map(|x| OneOrMany::One(Dynos::into_response(x))),
                None => {
                    if all.is_some_and(|x| x) {
                        if admin_query && is_admin {
                            dyno_actions::select_all(&mut conn)
                        } else {
                            dyno_actions::select_many(&mut conn, user_id)
                        }
                        .map(|x| {
                            OneOrMany::Many(
                                x.into_iter().map(Dynos::into_response).collect::<Vec<_>>(),
                            )
                        })
                    } else {
                        dyno_actions::select_many_limit(&mut conn, user_id, max.unwrap_or(5)).map(
                            |x| {
                                OneOrMany::Many(
                                    x.into_iter().map(Dynos::into_response).collect::<Vec<_>>(),
                                )
                            },
                        )
                    }
                }
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    blk_result.map(|x| match x {
        OneOrMany::One(one) => HttpResponse::Ok().json(ApiResponse::success(one)),
        OneOrMany::Many(many) => HttpResponse::Ok().json(ApiResponse::success(many)),
    })
}

use dyno_core::serde;
#[repr(u8)]
#[derive(Clone, Copy, Default, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(crate = "serde")]
pub enum FileType {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "bin")]
    #[default]
    Bin,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "excel")]
    Excel,
}

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize, PartialEq)]
#[serde(crate = "serde")]
pub struct QueryFile {
    #[serde(default)]
    pub tp: FileType,
}

#[get("/dyno/{user_uuid}/{file}")]
pub async fn get_file(
    web::Query(QueryFile { tp }): web::Query<QueryFile>,
    path: Path<(String, String)>,
    JwtUserMiddleware(_session): JwtUserMiddleware,
    data: web::Data<crate::ServerState>,
) -> DynoResult<HttpResponse> {
    let (user_uuid, file) = path.into_inner();
    let dyno_path = data
        .cfg
        .app_public_path
        .join("dyno")
        .join(user_uuid)
        .join(&file);

    web::block(move || {
        std::fs::read(dyno_path)
            .map_err(DynoErr::internal_server_error)
            .and_then(|bytes| match tp {
                FileType::Bin => Ok(bytes),
                FileType::Csv => {
                    BufferData::decompress(bytes).and_then(|x| x.save_csv_into_bytes())
                }
                FileType::Excel => {
                    BufferData::decompress(bytes).and_then(|x| x.save_excel_into_bytes())
                }
                FileType::Json => BufferData::decompress(bytes).and_then(|x| {
                    dyno_core::serde_json::to_vec(&ApiResponse::success(x))
                        .map_err(DynoErr::serialize_error)
                }),
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?
    .map(|data| match tp {
        FileType::Bin => HttpResponse::Ok()
            .append_header(header::ContentDisposition::attachment(file))
            .content_type("application/octet-stream")
            .body(data),
        FileType::Csv => HttpResponse::Ok()
            .append_header(header::ContentDisposition::attachment(format!(
                "{}.csv",
                file
            )))
            .content_type("text/csv")
            .body(data),
        FileType::Excel => HttpResponse::Ok()
            .append_header(header::ContentDisposition::attachment(format!(
                "{}.xlsx",
                file
            )))
            .content_type("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
            .body(data),
        FileType::Json => HttpResponse::Ok()
            .content_type("application/json")
            .body(data),
    })
}
