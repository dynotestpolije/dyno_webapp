use std::future::{ready, Ready};

use actix_web::{dev::Payload, http, web, FromRequest, HttpMessage, HttpRequest};
use dyno_core::{crypto::TokenDetails, model::UserSession, DynoErr, DynoResult};

fn jwt_from_req(req: &HttpRequest) -> DynoResult<UserSession> {
    let Some(data) = req.app_data::<web::Data<crate::ServerState>>() else {
        return Err(DynoErr::internal_server_error("No ServerState Data"));
    };
    let cfg = &data.cfg;

    match req
        .cookie("access_token")
        .as_ref()
        .map(|c| c.value())
        .or_else(|| {
            req.headers()
                .get(http::header::AUTHORIZATION)
                .map(http::header::HeaderValue::as_bytes)
                .and_then(|h| {
                    if h.starts_with(b"Bearer ") {
                        std::str::from_utf8(&h[7..]).ok()
                    } else {
                        None
                    }
                })
        }) {
        Some(tok) => match TokenDetails::verify(tok, cfg.jwt.access_token_public_key.as_bytes()) {
            Ok(token_details) => Ok(token_details.user),
            Err(err) => Err(DynoErr::unauthorized_error(format!(
                "Invalid Token - {err}"
            ))),
        },
        None => Err(DynoErr::unauthorized_error(
            "You are not logged in, please provide token",
        )),
    }
}

pub struct JwtUserMiddleware(pub UserSession);
pub struct JwtAdminMiddleware(pub UserSession);

impl FromRequest for JwtUserMiddleware {
    type Error = DynoErr;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(jwt_from_req(req).map(Self))
    }
}

impl FromRequest for JwtAdminMiddleware {
    type Error = DynoErr;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match jwt_from_req(req) {
            Ok(sess) if sess.role.is_admin() => {
                req.extensions_mut().insert(sess);
                ready(Ok(Self(sess)))
            }
            Ok(_) => ready(Err(DynoErr::forbidden_error("Admin Access Required!"))),
            Err(err) => ready(Err(err)),
        }
    }
}
