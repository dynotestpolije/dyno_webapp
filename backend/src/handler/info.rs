use actix_web::{get, web, HttpResponse, Responder};
use dyno_core::{users::OneOrMany, ApiResponse, DynoErr};

use crate::{actions, handler::DynoUrlsQueries, middlewares::JwtUserMiddleware};

#[get("/info")]
pub async fn get_info(
    web::Query(DynoUrlsQueries {
        id,
        all,
        max: _,
        admin,
    }): web::Query<DynoUrlsQueries>,
    JwtUserMiddleware(session): JwtUserMiddleware,
    data: web::Data<crate::ServerState>,
) -> impl Responder {
    let is_admin = session.role.is_admin();
    let admin_query = admin.is_some_and(|x| x);
    if admin.is_some_and(|x| x) && !is_admin {
        return Err(DynoErr::unauthorized_error(
            "NotAuthorized! Admin Access required!",
        ));
    }
    let ret = web::block(move || {
        data.db
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| match id {
                Some(id) => {
                    actions::info::select(&mut conn, id).map(|x| OneOrMany::One(x.into_response()))
                }
                None => {
                    if all.is_some_and(|x| x) && is_admin && admin_query {
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
