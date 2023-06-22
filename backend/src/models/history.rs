use dyno_core::{
    chrono::{NaiveDateTime, Utc},
    HistoryResponse,
};

use super::uuid::UUID;
use crate::schema::histories;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq, diesel::Queryable, diesel::Identifiable, diesel::Selectable)]
#[diesel(table_name = histories)]
pub struct History {
    pub id: i64,
    pub user_id: i64,
    pub user_uuid: UUID,
    pub dyno_id: i64,
    pub long_usage: i64,
    pub created_at: NaiveDateTime,
}

impl History {
    pub const fn to_response(self) -> HistoryResponse {
        let Self {
            id: _,
            user_id,
            user_uuid,
            dyno_id,
            long_usage,
            created_at,
        } = self;
        HistoryResponse {
            user_id,
            user_uuid: user_uuid.into_inner(),
            dyno_id,
            long_usage,
            created_at,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq, diesel::Insertable)]
#[diesel(table_name = histories)]
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
