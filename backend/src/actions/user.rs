use crate::models::user::{NewUser, UpdateUser, User};
use crate::schema::users::dsl;
use crate::DynoDBPooledConnection;
use diesel::prelude::*;
use dyno_core::{DynoErr, DynoResult};

macro_rules! impl_generic_funcs {
    ($($by:ident: $types:ty),*) => {
        dyno_core::paste::paste! {$(
            #[allow(unused)]
            #[inline]
            #[doc = " # Helper Function"]
            #[doc = "query find in database by " $by]
            #[doc = " # Return [DynoResult]"]
            #[doc = "Err([DynoErr::database_error])"]
            #[doc = "Ok([User])"]
            pub fn [<find_by_ $by:lower>](conn: &mut DynoDBPooledConnection, [<user_ $by:lower>]: $types) -> DynoResult<User> {
                super::query_one!(FIND users WHERE ($by.eq([<user_ $by:lower>])) as User [conn])
            }
            #[allow(unused)]
            #[inline]
            #[doc = " # Helper Function"]
            #[doc = "query delete record in database by " $by]
            #[doc = " # Return [DynoResult]"]
            #[doc = "Err([DynoErr::database_error])"]
            #[doc = "Ok([usize]) `number of rows that efected, not the id.`"]
            pub fn [<delete_by_ $by:lower>](conn: &mut DynoDBPooledConnection, [<user_ $by:lower>]: $types) -> DynoResult<usize> {
                super::query_one!(DELETE users WHERE ($by.eq([<user_ $by:lower>])) [conn])
            }
            #[allow(unused)]
            #[inline]
            #[doc = " # Helper Function"]
            #[doc = "query update record in database by " $by "with paramater [UpdateUser] as [AsChangeset]"]
            #[doc = " # Return [DynoResult]"]
            #[doc = "Err([DynoErr::database_error])"]
            #[doc = "Ok([usize]) `number of rows that efected, not the id.`"]
            pub fn [<update_by_ $by:lower>](conn: &mut DynoDBPooledConnection, [<user_ $by:lower>]: $types, updated: UpdateUser) -> DynoResult<usize> {
                super::query_one!(UPDATE users WHERE ($by.eq([<user_ $by:lower>])) VALUES updated [conn])
            }
        )*}
    };
}
impl_generic_funcs!(uuid: &str, id: i32, nim: &str);

#[allow(unused)]
#[inline]
pub fn is_exists_by_id(conn: &mut DynoDBPooledConnection, id: i32) -> DynoResult<bool> {
    dsl::users
        .find(id)
        .select(dsl::id)
        .first::<i32>(conn)
        .optional()
        .map_err(DynoErr::database_error)
        .map(|x| x.is_some())
}
#[allow(unused)]
#[inline]
pub fn is_exists_by_nim(conn: &mut DynoDBPooledConnection, nim: &str) -> DynoResult<bool> {
    dsl::users
        .select(dsl::id)
        .filter(dsl::nim.eq(nim))
        .first::<i32>(conn)
        .optional()
        .map_err(DynoErr::database_error)
        .map(|x| x.is_some())
}

#[allow(unused)]
#[inline]
pub fn insert_new(conn: &mut DynoDBPooledConnection, new: NewUser) -> DynoResult<usize> {
    diesel::insert_into(dsl::users)
        .values(new)
        .execute(conn)
        .map_err(DynoErr::database_error)
}

#[allow(unused)]
#[inline]
/// # Returns.
/// this function will return number of rows that efected, not id the user.
pub fn insert_many(conn: &mut DynoDBPooledConnection, news: Vec<NewUser>) -> DynoResult<usize> {
    diesel::insert_into(dsl::users)
        .values(news)
        .execute(conn)
        .map_err(DynoErr::database_error)
}

#[allow(unused)]
#[inline]
/// # PARAMETER
/// - limit: number of limit the selected rows, if [Option::None] given, defaulted to `5`
/// # Returns.
/// this function will return number of rows that efected, not id the user.
pub fn select_many(conn: &mut DynoDBPooledConnection, limit: Option<u32>) -> DynoResult<Vec<User>> {
    let limit = limit.unwrap_or(5) as _;
    dsl::users
        .limit(limit)
        .select(User::as_select())
        .load(conn)
        .map_err(DynoErr::database_error)
}
