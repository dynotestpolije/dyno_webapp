use crate::models::dyno::{Dynos, NewDynos};
use crate::DynoDBPooledConnection;
use diesel::prelude::*;
use dyno_core::{DynoErr, DynoResult};

#[inline]
#[allow(unused)]
pub fn is_exists(conn: &mut DynoDBPooledConnection, id: i32, user_id: i32) -> DynoResult<bool> {
    use crate::schema::dynos::dsl;
    dsl::dynos
        .filter(dsl::id.eq(id).and(dsl::user_id.eq(user_id)))
        .select(dsl::id)
        .first::<i32>(conn)
        .optional()
        .map_err(DynoErr::database_error)
        .map(|x| x.is_some())
}

#[inline]
#[allow(unused)]
pub fn select(conn: &mut DynoDBPooledConnection, id: i32, user_id: i32) -> DynoResult<Dynos> {
    use crate::schema::dynos;
    dynos::table
        .filter(dynos::dsl::id.eq(id).and(dynos::dsl::user_id.eq(user_id)))
        .select(Dynos::as_select())
        .first::<Dynos>(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn select_id(conn: &mut DynoDBPooledConnection, id: i32, user_id: i32) -> DynoResult<i32> {
    use crate::schema::dynos::dsl;
    dsl::dynos
        .filter(dsl::id.eq(id).and(dsl::user_id.eq(user_id)))
        .select(dsl::id)
        .first(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn insert(conn: &mut DynoDBPooledConnection, new: NewDynos) -> DynoResult<i32> {
    use crate::schema::dynos;
    diesel::insert_into(dynos::table)
        .values(new)
        .returning(dynos::dsl::id)
        .get_result::<i32>(conn)
        .map_err(DynoErr::database_error)
}

#[inline]
#[allow(unused)]
pub fn select_many(
    conn: &mut DynoDBPooledConnection,
    user_id: i32,
    limit: Option<u32>,
) -> DynoResult<Vec<Dynos>> {
    use crate::schema::dynos;

    let limit = limit.unwrap_or(5) as _;
    dynos::table
        .filter(dynos::dsl::user_id.eq(user_id))
        .select(Dynos::as_select())
        .limit(limit)
        .get_results::<Dynos>(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn insert_many(conn: &mut DynoDBPooledConnection, new: Vec<NewDynos>) -> DynoResult<usize> {
    use crate::schema::dynos;
    diesel::insert_into(dynos::table)
        .values(new)
        .execute(conn)
        .map_err(DynoErr::database_error)
}
