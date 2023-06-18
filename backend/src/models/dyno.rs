use crate::schema::dynos;
use dyno_core::chrono::NaiveDateTime;
use dyno_core::{
    dynotests::{DynoTest, DynoTestDataInfo},
    serde,
};

use super::uuid::UUID;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(
    Clone,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Selectable,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(crate = "serde")]
#[diesel(table_name = dynos)]
pub struct Dynos {
    pub id: i64,
    pub user_id: i64,
    pub info_id: Option<i64>,
    pub uuid: UUID,
    pub data_url: String,
    pub data_checksum: String,
    pub verified: Option<bool>,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Dynos {
    #[inline]
    pub fn into_response(self) -> DynoTest {
        DynoTest {
            id: self.id,
            user_id: self.user_id,
            info_id: self.info_id,
            uuid: self.uuid.into_inner(),
            data_url: self.data_url,
            data_checksum: self.data_checksum,
            verified: self.verified.is_some_and(|x| x),
            start: self.start,
            stop: self.stop,
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, diesel::Insertable)]
#[diesel(table_name = dynos)]
pub struct NewDynos {
    pub user_id: i64,
    pub info_id: Option<i64>,
    pub uuid: UUID,
    pub data_url: String,
    pub data_checksum: String,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
}

impl NewDynos {
    pub fn new(
        user_id: i64,
        info_id: Option<i64>,
        data_url: impl ToString,
        DynoTestDataInfo {
            checksum_hex: data_checksum,
            start,
            stop,
            ..
        }: DynoTestDataInfo,
    ) -> Self {
        Self {
            uuid: UUID::new(),
            data_url: data_url.to_string(),
            user_id,
            info_id,
            data_checksum,
            start,
            stop,
        }
    }
}
