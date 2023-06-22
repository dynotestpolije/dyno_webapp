use actix_web::{get, web, HttpResponse, Responder};
use dyno_core::{users::OneOrMany, ApiResponse, DynoErr};

use crate::{actions, middlewares::JwtUserMiddleware};

use super::DynoInfoQueries;

#[get("/info")]
pub async fn get_info(
    web::Query(DynoInfoQueries { id, all, admin }): web::Query<DynoInfoQueries>,
    JwtUserMiddleware(session): JwtUserMiddleware,
    data: web::Data<crate::ServerState>,
) -> impl Responder {
    let ret = web::block(move || {
        data.db
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| match id {
                Some(id) => {
                    actions::info::select(&mut conn, id).map(|x| OneOrMany::One(x.into_response()))
                }
                None => {
                    if all.is_some_and(|x| x) && admin.is_some_and(|x| x) {
                        actions::info::select_all(&mut conn).map(|x| {
                            OneOrMany::Many(x.into_iter().map(|d| d.into_response()).collect())
                        })
                    } else {
                        actions::info::select_many(&mut conn, session.id).map(|x| {
                            OneOrMany::Many(x.into_iter().map(|d| d.into_response()).collect())
                        })
                    }
                }
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    match ret {
        Ok(OneOrMany::One(one)) => Ok(HttpResponse::Ok().json(ApiResponse::success(one))),
        Ok(OneOrMany::Many(many)) => Ok(HttpResponse::Ok().json(ApiResponse::success(many))),
        Err(err) => Err(err),
    }
}
