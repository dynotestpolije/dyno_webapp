use crate::schema::dynos;
use dyno_core::chrono::NaiveDateTime;
use dyno_core::dynotests::DynoTestDataInfo;
use dyno_core::serde;

use super::uuid::UUID;

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
#[diesel(table_name = dynos)]
pub struct Dynos {
    pub id: i32,
    pub user_id: i32,
    pub info_id: Option<i32>,
    pub uuid: UUID,
    pub data_checksum: String,
    pub verified: Option<bool>,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[serde(crate = "serde")]
#[diesel(table_name = dynos)]
pub struct NewDynos {
    pub user_id: i32,
    pub info_id: Option<i32>,
    pub uuid: UUID,
    pub data_checksum: String,
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
}

impl NewDynos {
    pub fn new(
        info_id: Option<i32>,
        user_id: i32,
        DynoTestDataInfo {
            checksum_hex: data_checksum,
            start,
            stop,
            ..
        }: DynoTestDataInfo,
    ) -> Self {
        Self {
            uuid: UUID::new(),
            user_id,
            info_id,
            data_checksum,
            start,
            stop,
        }
    }
}
