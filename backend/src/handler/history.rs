use actix_web::{get, web, HttpResponse, Responder};
use dyno_core::{ApiResponse, DynoErr};

use crate::{actions, middlewares::JwtUserMiddleware, models::history::History};

#[get("/history")]
pub async fn history(
    JwtUserMiddleware(session): JwtUserMiddleware,
    data: web::Data<crate::ServerState>,
) -> impl Responder {
    let id = session.id;
    let ret = web::block(move || {
        data.db
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| {
                actions::history::select_many(&mut conn, id)
                    .map(|x| x.into_iter().map(History::to_response).collect::<Vec<_>>())
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    match ret {
        Ok(ok) => Ok(HttpResponse::Ok().json(ApiResponse::success(ok))),
        Err(err) => Err(err),
    }
}
