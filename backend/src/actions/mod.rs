pub mod dyno;
pub mod info;
pub mod user;
pub mod history;

macro_rules! query_one {
    (FIND $table:ident WHERE ($filter:expr) as $types:ty [$conn:expr]) => {{
        use crate::schema::$table::dsl::*;
        $table
            .filter($filter)
            .select(<$types>::as_select())
            .first::<$types>($conn)
            .optional()
            .map_err(DynoErr::database_error)?
            .ok_or(DynoErr::database_error(concat!(
                stringify!($table),
                " when ",
                stringify!($filter),
                ", not exists in table"
            )))
    }};
    (UPDATE $table:ident WHERE ($filter:expr) VALUES $val:ident [$conn:expr]) => {{
        use crate::schema::$table::dsl::*;
        diesel::update($table.filter($filter))
            .set($val)
            .returning(id)
            .get_result::<i64>($conn)
            .map_err(DynoErr::database_error)
    }};
    (DELETE $table:ident WHERE ($filter:expr) [$conn:expr]) => {{
        use crate::schema::$table::dsl::*;
        diesel::delete($table.filter($filter))
            .returning(id)
            .get_result::<i64>($conn)
            .map_err(DynoErr::database_error)
    }};
}

pub(self) use query_one;
