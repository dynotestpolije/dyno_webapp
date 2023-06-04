pub mod auth;
pub mod dyno;
pub mod user;

#[cfg_attr(debug_assert, derive(Debug))]
#[derive(Clone, Default, dyno_core::serde::Deserialize)]
#[serde(crate = "dyno_core::serde")]
pub struct UserUrlsQueries {
    pub id: Option<i32>,
    pub max: Option<u32>,
}

#[cfg_attr(debug_assert, derive(Debug))]
#[derive(Clone, Default, dyno_core::serde::Deserialize)]
#[serde(crate = "dyno_core::serde")]
pub struct DynoUrlsQueries {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub max: Option<u32>,
}

#[actix_web::get("/health")]
pub async fn check_health() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(dyno_core::ApiResponse::success(
        "Dynotest api Health Check: OK".to_owned(),
    ))
}

#[inline]
pub fn api() -> actix_web::Scope {
    use actix_web::web;
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
        .service(dyno::get_dyno_file_path)
}
