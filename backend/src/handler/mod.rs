use actix_web::{guard::GuardContext, http::header, web::ServiceConfig};
use dyno_core::{ActiveResponse, ApiResponse, DynoConfig, DynoErr, DynoResult};

use crate::{actions, middlewares::JwtUserMiddleware, models::user::User};

pub mod auth;
pub mod dyno;
pub mod history;
pub mod info;
pub mod user;
pub mod ws;

#[inline]
pub fn api(conf: &mut ServiceConfig) {
    use actix_web::web;
    conf.service(
        web::scope("/api")
            .service(check_health)
            .service(auth::me)
            .service(auth::register_user)
            .service(auth::login_user)
            .service(auth::logout_user)
            .service(user::get_user)
            .service(user::add_user)
            .service(user::update_user)
            .service(user::delete_user)
            .service(dyno::get_dyno)
            .service(dyno::add_dyno)
            .service(history::history)
            .service(info::get_info)
            .service(get_active)
            .service(post_active)
            .service(post_non_active),
    )
    .service(ws::websocket_endpoint)
    .service(dyno::get_file);
}

#[cfg_attr(debug_assert, derive(Debug))]
#[derive(Clone, Default, dyno_core::serde::Deserialize)]
#[serde(crate = "dyno_core::serde")]
pub struct UserUrlsQueries {
    pub id: Option<i64>,
    pub max: Option<u32>,
}

#[cfg_attr(debug_assert, derive(Debug))]
#[derive(Clone, Default, dyno_core::serde::Deserialize)]
#[serde(crate = "dyno_core::serde")]
pub struct DynoUrlsQueries {
    pub id: Option<i64>,
    pub max: Option<i64>,
    pub all: Option<bool>,
    pub admin: Option<bool>,
}

#[actix_web::get("/health")]
pub async fn check_health() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(dyno_core::ApiResponse::success(
        "Dynotest api Health Check: OK".to_owned(),
    ))
}

#[actix_web::get("/active")]
pub async fn get_active(
    JwtUserMiddleware(_session): JwtUserMiddleware,
    data: actix_web::web::Data<crate::ServerState>,
) -> impl actix_web::Responder {
    let Some(active) = data.get_active() else {
        return Err(DynoErr::not_found_error("No Active Use in Dynotest"));
    };

    let user = if let Some(user) = active.user {
        let active_user_id = user.id;
        Some(
            actix_web::web::block(move || {
                data.db
                    .get()
                    .map_err(DynoErr::database_error)
                    .and_then(|mut conn| {
                        actions::user::find_by_id(&mut conn, active_user_id)
                            .map(User::into_user_response)
                    })
            })
            .await
            .map_err(DynoErr::internal_server_error)??,
        )
    } else {
        None
    };

    DynoResult::Ok(
        actix_web::HttpResponse::Ok().json(ApiResponse::success(ActiveResponse {
            user,
            dyno: active.dyno,
            start: active.start,
        })),
    )
}

#[inline]
pub fn guard_desktop(guard_ctx: &GuardContext) -> bool {
    match guard_ctx.head().headers().get(header::USER_AGENT) {
        Some(ok) => ok.to_str().is_ok_and(|x| x.contains("Dyno/Desktop")),
        None => false,
    }
}

#[actix_web::post("/active", guard = "guard_desktop")]
pub async fn post_active(
    actix_web::web::Json(conf): actix_web::web::Json<DynoConfig>,
    data: actix_web::web::Data<crate::ServerState>,
) -> impl actix_web::Responder {
    data.change_active_dyno(conf);
    actix_web::HttpResponse::Ok().finish()
}

#[actix_web::post("/non_active", guard = "guard_desktop")]
pub async fn post_non_active(
    data: actix_web::web::Data<crate::ServerState>,
) -> impl actix_web::Responder {
    data.set_active(None);
    actix_web::HttpResponse::Ok().finish()
}
