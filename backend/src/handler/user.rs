use crate::{
    actions::user as user_actions,
    handler::UserUrlsQueries,
    middlewares::JwtAdminMiddleware,
    models::user::{NewUser, UpdateUser, User},
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use dyno_core::{
    model::ApiResponse,
    users::{OneOrMany, UserRegistration, UserUpdate},
    DynoErr,
};

#[get("/users")]
pub async fn get_user(
    web::Query(UserUrlsQueries { id, max }): web::Query<UserUrlsQueries>,
    _: JwtAdminMiddleware,
    data: web::Data<crate::ServerState>,
) -> impl Responder {
    let dbpool = data.db.clone();

    let user_response = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| match id {
                Some(id) => user_actions::find_by_id(&mut conn, id)
                    .map(|u| OneOrMany::One(u.into_user_response())),
                None => user_actions::select_many(&mut conn, max).map(|v| {
                    OneOrMany::Many(v.into_iter().map(User::into_user_response).collect())
                }),
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;
    match user_response {
        Ok(OneOrMany::One(one)) => Ok(HttpResponse::Ok().json(ApiResponse::success(one))),
        Ok(OneOrMany::Many(many)) => Ok(HttpResponse::Ok().json(ApiResponse::success(many))),
        Err(err) => Err(err),
    }
}

#[post("/users")]
pub async fn add_user(
    web::Json(jsons): web::Json<OneOrMany<UserRegistration>>,
    _: JwtAdminMiddleware,
    data: web::Data<crate::ServerState>,
) -> impl Responder {
    let dbpool = data.db.clone();
    let blk_result = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| match jsons {
                OneOrMany::One(user) => NewUser::from_registration(user)
                    .and_then(|new| user_actions::insert_new(&mut conn, new)),
                OneOrMany::Many(manys) => {
                    let many = manys
                        .into_iter()
                        .filter_map(|user| NewUser::from_registration(user).ok())
                        .collect::<Vec<_>>();
                    user_actions::insert_many(&mut conn, many).map_err(DynoErr::database_error)
                }
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;
    match blk_result {
        Ok(efected_row) => Ok(HttpResponse::Ok().json(ApiResponse::success(efected_row))),
        Err(err) => Err(err),
    }
}

#[patch("/users/{user_id}")]
pub async fn update_user(
    user_id: web::Path<u32>,
    web::Json(user_update): web::Json<UserUpdate>,
    _: JwtAdminMiddleware,
    data: web::Data<crate::ServerState>,
) -> impl Responder {
    let dbpool = data.db.clone();
    let id = user_id.into_inner();
    let user_response = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| {
                user_actions::update_by_id(&mut conn, id as _, UpdateUser::from(user_update))
            })
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    match user_response {
        Ok(ok) => Ok(HttpResponse::Ok().json(ApiResponse::success(ok))),
        Err(err) => Err(err),
    }
}

#[delete("/users/{user_id}")]
pub async fn delete_user(
    user_id: web::Path<u32>,
    _: JwtAdminMiddleware,
    data: web::Data<crate::ServerState>,
) -> impl Responder {
    let dbpool = data.db.clone();
    let id = user_id.into_inner();
    let user_response = web::block(move || {
        dbpool
            .get()
            .map_err(DynoErr::database_error)
            .and_then(|mut conn| user_actions::delete_by_id(&mut conn, id as _))
    })
    .await
    .map_err(DynoErr::internal_server_error)?;

    match user_response {
        Ok(ok) => Ok(HttpResponse::Ok().json(ApiResponse::success(ok))),
        Err(err) => Err(err),
    }
}
