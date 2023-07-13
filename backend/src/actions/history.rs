use crate::schema::histories::dsl;
use crate::{
    models::history::{History, NewHistory},
    DynoDBPooledConnection,
};
use diesel::prelude::*;
use dyno_core::{DynoErr, DynoResult};

#[inline]
#[allow(unused)]
pub fn is_exists(conn: &mut DynoDBPooledConnection) -> Option<i64> {
    dsl::histories
        .select(dsl::id)
        .first::<i64>(conn)
        .optional()
        .unwrap_or(None)
}

#[inline]
#[allow(unused)]
pub fn select(conn: &mut DynoDBPooledConnection, id: i64) -> DynoResult<History> {
    dsl::histories
        .filter(dsl::id.eq(id))
        .select(History::as_select())
        .first(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn insert(conn: &mut DynoDBPooledConnection, new: NewHistory) -> DynoResult<i64> {
    if let Some(id) = is_exists(conn) {
        return Ok(id);
    }
    diesel::insert_into(dsl::histories)
        .values(new)
        .returning(dsl::id)
        .get_result::<i64>(conn)
        .map_err(DynoErr::database_error)
}

#[inline]
#[allow(unused)]
pub fn select_many(
    conn: &mut DynoDBPooledConnection,
    id: i64,
    limit: Option<i64>,
) -> DynoResult<Vec<History>> {
    let mut query = dsl::histories
        .select(History::as_select())
        .filter(dsl::user_id.eq(id));
    if let Some(limit) = limit {
        let query = query.limit(limit);
    }
    query
        .get_results(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn select_all(conn: &mut DynoDBPooledConnection) -> DynoResult<Vec<History>> {
    dsl::histories
        .select(History::as_select())
        .get_results(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}
