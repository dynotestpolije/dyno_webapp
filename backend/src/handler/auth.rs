use crate::middlewares::JwtUserMiddleware;
use crate::{actions::user as user_actions, models::user::NewUser};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    http, web, HttpResponse, Responder,
};
use actix_web::{get, post};
use dyno_core::{
    crypto::{jwt_encode, verify_hash_password},
    model::users::{UserLogin, UserRegistration},
    ApiResponse, DynoErr, TokenClaims, UserSession,
};

const RESET_AGE_DUR: ActixWebDuration = ActixWebDuration::new(-1, 0);

#[post("/auth/register")]
pub async fn register_user(
    web::Json(user_registration): web::Json<UserRegistration>,
    dbpool: web::Data<crate::DynoDBPool>,
) -> impl Responder {
    let ret_block = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| {
                if !matches!(
                    user_actions::is_exists_by_nim(&mut conn, &user_registration.nim),
                    Ok(true)
                ) {
                    return Err(DynoErr::bad_request_error("User is already registered!"));
                }
                let newuser = NewUser::from_registration(user_registration)?;
                user_actions::insert_new(&mut conn, newuser)
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;
    match ret_block {
        Ok(ok) => Ok(HttpResponse::Ok().json(ApiResponse::success(ok))),
        Err(err) => Err(err),
    }
}

#[post("/auth/login")]
pub async fn login_user(
    web::Json(UserLogin { nim, password }): web::Json<UserLogin>,
    dbpool: web::Data<crate::DynoDBPool>,
    cfg: web::Data<crate::config::ServerConfig>,
) -> impl Responder {
    let nim_borrowed = nim.clone();
    let user = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| user_actions::find_by_nim(&mut conn, &nim_borrowed))
    })
    .await
    .map_err(DynoErr::internal_server_error)??;

    if !verify_hash_password(user.password, password) {
        return Err(DynoErr::unauthorized_error(
            "Auth Failed! User Password is not the same",
        ));
    }

    let claims = TokenClaims::new(UserSession {
        id: user.id,
        uuid: user.uuid.uuid()?,
        role: user.role.into_inner(),
    });

    let token = jwt_encode(claims, cfg.jwt.secret.as_bytes())?;
    let header_token = http::header::HeaderValue::from_str(&format!("Bearer {}", token))
        .map_err(DynoErr::internal_server_error)?;

    Ok(HttpResponse::Ok()
        .cookie(
            Cookie::build("token", token.clone())
                .path("/")
                .max_age(ActixWebDuration::new(60 * 60, 0))
                .http_only(true)
                .finish(),
        )
        .append_header((http::header::AUTHORIZATION, header_token))
        .json(ApiResponse::success(token)))
}

#[get("/auth/logout")]
pub async fn logout_user(JwtUserMiddleware(_d): JwtUserMiddleware) -> impl Responder {
    HttpResponse::Ok()
        .cookie(
            Cookie::build("token", "")
                .path("/")
                .max_age(RESET_AGE_DUR)
                .http_only(true)
                .finish(),
        )
        .json(ApiResponse::<String>::success("Logout Success".to_owned()))
}

#[get("/auth/me")]
pub async fn me(
    JwtUserMiddleware(user_session): JwtUserMiddleware,
    dbpool: web::Data<crate::DynoDBPool>,
) -> impl Responder {
    let user_response = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| user_actions::find_by_id(&mut conn, user_session.id))
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    match user_response {
        Ok(u) => Ok(HttpResponse::Ok().json(ApiResponse::success(u))),
        Err(err) => Err(err),
    }
}
