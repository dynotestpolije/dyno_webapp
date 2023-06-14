use crate::middlewares::JwtUserMiddleware;
use crate::{actions::user as user_actions, models::user::NewUser};
use actix_web::cookie::{self, Cookie};
use actix_web::{get, post};
use actix_web::{web, HttpResponse};
use dyno_core::crypto::TokenDetails;
use dyno_core::DynoResult;
use dyno_core::{
    crypto::verify_hash_password,
    model::users::{UserLogin, UserRegistration},
    ApiResponse, DynoErr, UserSession,
};

const RESET_AGE_DUR: cookie::time::Duration = cookie::time::Duration::new(-1, 0);

#[post("/auth/register")]
pub async fn register_user(
    web::Json(user_registration): web::Json<UserRegistration>,
    data: web::Data<crate::ServerState>,
) -> DynoResult<HttpResponse> {
    let ret_block = web::block(move || {
        data.db
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
    data: web::Data<crate::ServerState>,
) -> DynoResult<HttpResponse> {
    dyno_core::log::debug!("login endpoint post with nim: {nim}, pswd: {password}");
    let nim_borrowed = nim.clone();
    let db = data.db.clone();
    let user = web::block(move || {
        db.get()
            .map_err(DynoErr::database_error)
            .and_then(
                |mut conn| match user_actions::find_by_nim(&mut conn, &nim_borrowed) {
                    Ok(user) if verify_hash_password(&user.password, password) => Ok(user),
                    Ok(_) => Err(DynoErr::unauthorized_error(
                        "Auth Failed! User Password is not the same",
                    )),
                    Err(err) => Err(err),
                },
            )
    })
    .await
    .map_err(DynoErr::internal_server_error)??;

    let user_session = UserSession {
        id: user.id,
        role: user.role.into_inner(),
        uuid: user.uuid.into_inner(),
    };

    let access_token_details = TokenDetails::generate(
        user_session,
        data.cfg.jwt.access_token_max_age,
        data.cfg.jwt.access_token_private_key.as_bytes(),
    )?;
    let refresh_token_details = TokenDetails::generate(
        user_session,
        data.cfg.jwt.refresh_token_max_age,
        data.cfg.jwt.refresh_token_private_key.as_bytes(),
    )?;

    let access_cookie = Cookie::build("access_token", access_token_details.token.clone().unwrap())
        .path("/")
        .max_age(cookie::time::Duration::minutes(
            data.cfg.jwt.access_token_max_age,
        ))
        .http_only(true)
        .finish();
    let refresh_cookie = Cookie::build("refresh_token", refresh_token_details.token.unwrap())
        .path("/")
        .max_age(cookie::time::Duration::minutes(
            data.cfg.jwt.refresh_token_max_age,
        ))
        .http_only(true)
        .finish();

    let logged_in_cookie = Cookie::build("logged_in", "true")
        .path("/")
        .max_age(cookie::time::Duration::minutes(
            data.cfg.jwt.access_token_max_age,
        ))
        .http_only(false)
        .finish();

    dyno_core::DynoResult::Ok(
        HttpResponse::Ok()
            .cookie(access_cookie)
            .cookie(refresh_cookie)
            .cookie(logged_in_cookie)
            .json(ApiResponse::success(access_token_details)),
    )
}

#[get("/auth/logout")]
pub async fn logout_user(JwtUserMiddleware(_d): JwtUserMiddleware) -> HttpResponse {
    let access_cookie = Cookie::build("access_token", "")
        .path("/")
        .max_age(RESET_AGE_DUR)
        .http_only(true)
        .finish();
    let refresh_cookie = Cookie::build("refresh_token", "")
        .path("/")
        .max_age(RESET_AGE_DUR)
        .http_only(true)
        .finish();
    let logged_in_cookie = Cookie::build("logged_in", "")
        .path("/")
        .max_age(RESET_AGE_DUR)
        .http_only(true)
        .finish();
    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .cookie(logged_in_cookie)
        .json(ApiResponse::<String>::success("Logout Success".to_owned()))
}

#[get("/auth/me")]
pub async fn me(
    JwtUserMiddleware(user_session): JwtUserMiddleware,
    data: web::Data<crate::ServerState>,
) -> DynoResult<HttpResponse> {
    let user_response = web::block(move || {
        data.db
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
