use dyno_core::chrono::{NaiveDateTime, Utc};

use super::uuid::UUID;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(
    Clone,
    Copy,
    PartialEq,
    // diesel::Queryable,
    // diesel::Identifiable,
    // diesel::Selectable,
)]
// #[diesel(table_name = dyno_info)]
pub struct History {
    pub id: i64,
    pub user_id: i64,
    pub user_uuid: UUID,
    pub dyno_id: i64,
    pub long_usage: i64,
    pub created_at: NaiveDateTime,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(
    Clone,
    Copy,
    PartialEq,
    // diesel::Insertable
)]
// #[diesel(table_name = dyno_info)]
pub struct NewHistory {
    pub user_id: i64,
    pub user_uuid: UUID,
    pub dyno_id: i64,
    pub long_usage: i64,
    pub created_at: NaiveDateTime,
}

impl NewHistory {
    #[allow(unused)]
    pub fn new(user_id: i64, user_uuid: impl Into<UUID>, dyno_id: i64, long_usage: i64) -> Self {
        Self {
            user_id,
            user_uuid: user_uuid.into(),
            dyno_id,
            long_usage,
            created_at: Utc::now().naive_utc(),
        }
    }
}
