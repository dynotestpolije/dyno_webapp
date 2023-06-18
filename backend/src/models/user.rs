use crate::schema::users;
use dyno_core::chrono::{NaiveDateTime, Utc};
use dyno_core::users::{UserRegistration, UserResponse, UserUpdate};
use dyno_core::{serde, DynoResult};

use super::{role::ROLES, uuid::UUID};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Selectable,
)]
#[serde(crate = "serde")]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub uuid: UUID,
    pub nim: String,
    pub name: String,
    pub password: String,
    pub role: ROLES,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn into_user_response(self) -> UserResponse {
        UserResponse {
            id: self.id as _,
            uuid: self.uuid.into_inner(),
            nim: self.nim,
            name: self.name,
            email: self.email,
            photo: self.photo,
            role: self.role.into_inner(),
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[serde(crate = "serde")]
#[diesel(table_name = users)]
#[diesel(treat_none_as_default_value = false)]
pub struct NewUser {
    pub uuid: UUID,
    pub nim: String,
    pub name: String,
    pub password: String,
    pub role: ROLES,
    pub email: Option<String>,
    pub photo: Option<String>,
}

impl NewUser {
    pub fn from_registration(
        UserRegistration {
            nim,
            email,
            password,
            role,
            ..
        }: UserRegistration,
    ) -> DynoResult<Self> {
        dyno_core::crypto::hash_password(password).map(|hashed_pswd| Self {
            uuid: UUID::new(),
            name: nim.clone(),
            nim,
            password: hashed_pswd,
            role: ROLES(role),
            email: Some(email),
            photo: None,
        })
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Default, serde::Serialize, serde::Deserialize, diesel::AsChangeset)]
#[serde(crate = "serde")]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub nim: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub role: Option<ROLES>,
    pub email: Option<Option<String>>,
    pub photo: Option<Option<String>>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<UserUpdate> for UpdateUser {
    fn from(
        UserUpdate {
            nim,
            name,
            role,
            email,
            photo,
            password,
        }: UserUpdate,
    ) -> Self {
        Self {
            nim,
            name,
            password,
            role: role.map(ROLES),
            email: Some(email),
            photo: Some(photo),
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}
