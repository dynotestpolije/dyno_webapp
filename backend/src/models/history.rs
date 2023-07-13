use crate::schema::histories;
use dyno_core::{
    chrono::{NaiveDateTime, Utc},
    HistoryResponse,
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq, diesel::Queryable, diesel::Identifiable, diesel::Selectable)]
#[diesel(table_name = histories)]
pub struct History {
    pub id: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
}

impl History {
    pub const fn into_response(self) -> HistoryResponse {
        let Self {
            id,
            user_id,
            created_at,
        } = self;
        HistoryResponse {
            id,
            user_id,
            created_at,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq, diesel::Insertable)]
#[diesel(table_name = histories)]
pub struct NewHistory {
    pub user_id: i64,
    pub created_at: NaiveDateTime,
}

impl NewHistory {
    #[allow(unused)]
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            created_at: Utc::now().naive_utc(),
        }
    }
}
