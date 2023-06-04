use std::future::{ready, Ready};

use actix_web::{dev::Payload, http, web, FromRequest, HttpMessage, HttpRequest};
use dyno_core::{
    crypto::jwt_decode,
    model::{TokenClaims, UserSession},
    DynoErr, DynoResult,
};

use crate::config::ServerConfig;

fn jwt_from_req(req: &HttpRequest) -> DynoResult<TokenClaims> {
    let Some(cfg) = req.app_data::<web::Data<ServerConfig>>() else {
        return Err(DynoErr::internal_server_error("No ServerState Data"));
    };
    match req.cookie("token").as_ref().map(|c| c.value()).or_else(|| {
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
        Some(tok) => match jwt_decode::<TokenClaims>(tok, cfg.jwt.secret.as_bytes()) {
            Ok(c) => Ok(c),
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

impl FromRequest for JwtUserMiddleware {
    type Error = DynoErr;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match jwt_from_req(req) {
            Ok(tok) => {
                req.extensions_mut().insert(tok.sub.clone());
                ready(Ok(Self(tok.sub)))
            }
            Err(err) => ready(Err(err)),
        }
    }
}

pub struct JwtAdminMiddleware(pub UserSession);

impl FromRequest for JwtAdminMiddleware {
    type Error = DynoErr;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match jwt_from_req(req) {
            Ok(tok) if tok.sub.role.is_admin() => {
                req.extensions_mut().insert(tok.sub.clone());
                ready(Ok(Self(tok.sub)))
            }
            Ok(_) => ready(Err(DynoErr::forbidden_error("Admin Access Required!"))),
            Err(err) => ready(Err(err)),
        }
    }
}
