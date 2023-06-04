use crate::schema::dyno_info::dsl;
use crate::{
    models::info::{DynoInfo, NewDynoInfo},
    DynoDBPooledConnection,
};
use diesel::prelude::*;
use dyno_core::{DynoErr, DynoResult};

#[inline]
#[allow(unused)]
pub fn is_exists(
    conn: &mut DynoDBPooledConnection,
    id: i32,
    name: impl AsRef<str>,
) -> DynoResult<bool> {
    let name = name.as_ref();
    dsl::dyno_info
        .select(dsl::id)
        .filter(dsl::id.eq(id).and(dsl::name.eq(name)))
        .first::<i32>(conn)
        .optional()
        .map_err(DynoErr::database_error)
        .map(|x| x.is_some())
}

#[inline]
#[allow(unused)]
pub fn select(conn: &mut DynoDBPooledConnection, id: i32) -> DynoResult<DynoInfo> {
    dsl::dyno_info
        .filter(dsl::id.eq(id))
        .select(DynoInfo::as_select())
        .first::<DynoInfo>(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn insert(conn: &mut DynoDBPooledConnection, new: NewDynoInfo) -> DynoResult<i32> {
    diesel::insert_into(dsl::dyno_info)
        .values(new)
        .returning(dsl::id)
        .get_result::<i32>(conn)
        .map_err(DynoErr::database_error)
}

#[inline]
#[allow(unused)]
pub fn select_many(conn: &mut DynoDBPooledConnection, id: i32) -> DynoResult<Vec<DynoInfo>> {
    dsl::dyno_info
        .select(DynoInfo::as_select())
        .filter(dsl::id.eq(id))
        .get_results::<DynoInfo>(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn insert_many(conn: &mut DynoDBPooledConnection, new: Vec<NewDynoInfo>) -> DynoResult<usize> {
    diesel::insert_into(dsl::dyno_info)
        .values(new)
        .execute(conn)
        .map_err(DynoErr::database_error)
}