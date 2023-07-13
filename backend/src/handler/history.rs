use actix_web::{get, web, HttpResponse, Responder};
use dyno_core::{users::OneOrMany, ApiResponse, DynoErr};

use crate::{
    actions::history as history_actions, handler::DynoUrlsQueries, middlewares::JwtUserMiddleware,
    models::history::History,
};

#[get("/history")]
pub async fn history(
    web::Query(DynoUrlsQueries {
        id,
        all,
        max,
        admin,
    }): web::Query<DynoUrlsQueries>,
    JwtUserMiddleware(session): JwtUserMiddleware,
    data: web::Data<crate::ServerState>,
) -> impl Responder {
    let is_admin = session.role.is_admin();
    let admin_query = admin.is_some_and(|x| x);
    if admin_query && !is_admin {
        return Err(DynoErr::unauthorized_error(
            "NotAuthorized! Admin Access required!",
        ));
    }
    let user_id = session.id;
    let ret = web::block(move || {
        data.db
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| match id {
                Some(id) => history_actions::select(&mut conn, id)
                    .map(|x| OneOrMany::One(History::into_response(x))),
                None => {
                    if all.is_some_and(|x| x) {
                        if admin_query && is_admin {
                            history_actions::select_all(&mut conn)
                        } else {
                            history_actions::select_many(&mut conn, user_id, max)
                        }
                        .map(|x| {
                            OneOrMany::Many(
                                x.into_iter()
                                    .map(History::into_response)
                                    .collect::<Vec<_>>(),
                            )
                        })
                    } else {
                        history_actions::select_many(&mut conn, user_id, Some(max.unwrap_or(5)))
                            .map(|x| {
                                OneOrMany::Many(
                                    x.into_iter()
                                        .map(History::into_response)
                                        .collect::<Vec<_>>(),
                                )
                            })
                    }
                }
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    match ret {
        Ok(OneOrMany::One(one)) => Ok(HttpResponse::Ok().json(ApiResponse::success(one))),
        Ok(OneOrMany::Many(one)) => Ok(HttpResponse::Ok().json(ApiResponse::success(one))),
        Err(err) => Err(err),
    }
}
