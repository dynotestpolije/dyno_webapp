use crate::models::dyno::{Dynos, NewDynos};
use crate::DynoDBPooledConnection;
use diesel::prelude::*;
use dyno_core::{DynoErr, DynoResult};

#[inline]
#[allow(unused)]
pub fn get_last_id(conn: &mut DynoDBPooledConnection) -> DynoResult<i64> {
    use crate::schema::dynos::dsl;
    dsl::dynos
        .select(dsl::id)
        .order(dsl::id.desc())
        .first::<i64>(conn)
        .optional()
        .map_err(DynoErr::database_error)
        .map(|x| x.unwrap_or(1))
}

#[inline]
#[allow(unused)]
pub fn is_exists(conn: &mut DynoDBPooledConnection, id: i64, user_id: i64) -> DynoResult<bool> {
    use crate::schema::dynos::dsl;
    dsl::dynos
        .filter(dsl::id.eq(id).and(dsl::user_id.eq(user_id)))
        .select(dsl::id)
        .first::<i64>(conn)
        .optional()
        .map_err(DynoErr::database_error)
        .map(|x| x.is_some())
}

#[inline]
#[allow(unused)]
pub fn select(conn: &mut DynoDBPooledConnection, id: i64, user_id: i64) -> DynoResult<Dynos> {
    use crate::schema::dynos;
    dynos::table
        .filter(dynos::dsl::id.eq(id).and(dynos::dsl::user_id.eq(user_id)))
        .select(Dynos::as_select())
        .get_result(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}
#[inline]
#[allow(unused)]
pub fn select_by_id(conn: &mut DynoDBPooledConnection, id: i64) -> DynoResult<Dynos> {
    use crate::schema::dynos;
    dynos::table
        .filter(dynos::dsl::id.eq(id))
        .select(Dynos::as_select())
        .get_result(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn select_id(conn: &mut DynoDBPooledConnection, id: i64, user_id: i64) -> DynoResult<i64> {
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
pub fn insert(conn: &mut DynoDBPooledConnection, new: NewDynos) -> DynoResult<i64> {
    use crate::schema::dynos;
    diesel::insert_into(dynos::table)
        .values(new)
        .returning(dynos::dsl::id)
        .get_result::<i64>(conn)
        .map_err(DynoErr::database_error)
}

#[inline]
#[allow(unused)]
pub fn select_many(conn: &mut DynoDBPooledConnection, user_id: i64) -> DynoResult<Vec<Dynos>> {
    use crate::schema::dynos;
    dynos::table
        .filter(dynos::dsl::user_id.eq(user_id))
        .select(Dynos::as_select())
        .get_results::<Dynos>(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn select_all(conn: &mut DynoDBPooledConnection) -> DynoResult<Vec<Dynos>> {
    use crate::schema::dynos;
    dynos::table
        .select(Dynos::as_select())
        .get_results::<Dynos>(conn)
        .optional()
        .map_err(DynoErr::database_error)?
        .ok_or(DynoErr::database_error("Dynos record not exists in table"))
}

#[inline]
#[allow(unused)]
pub fn select_many_limit(
    conn: &mut DynoDBPooledConnection,
    user_id: i64,
    limit: i64,
) -> DynoResult<Vec<Dynos>> {
    use crate::schema::dynos;

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
