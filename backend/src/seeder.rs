use crate::{
    actions,
    models::{role, user, uuid},
};
use dyno_core::{log, DynoResult};

fn seed_user(conn: &mut crate::DynoDBPooledConnection) -> DynoResult<()> {
    let password = dyno_core::crypto::hash_password("password123")?;
    let new_user = user::NewUser {
        uuid: uuid::UUID::new(),
        nim: "e32201406".to_owned(),
        name: "rizal".to_owned(),
        password,
        role: role::ROLES(dyno_core::role::Roles::Admin),
        email: Some("e32201406@student.polije.ac.id".to_owned()),
        photo: None,
    };
    if !matches!(actions::user::is_exists_by_id(conn, 1), Ok(true)) {
        log::debug!(
            "seeding user in databases for debug purposes: {:?}",
            &new_user
        );

        actions::user::insert_new(conn, new_user)?;
    }

    let password = dyno_core::crypto::hash_password("12345678")?;
    let new_user = user::NewUser {
        uuid: uuid::UUID::new(),
        nim: "ujicoba".to_owned(),
        name: "user uji coba".to_owned(),
        password,
        role: role::ROLES(dyno_core::role::Roles::User),
        email: Some("ujicoba@email.com".to_owned()),
        photo: None,
    };

    if !matches!(actions::user::is_exists_by_nim(conn, "ujicoba"), Ok(true)) {
        log::debug!(
            "seeding user in databases for debug purposes: {:?}",
            &new_user
        );
        actions::user::insert_new(conn, new_user)?;
    }

    Ok(())
}

pub fn seeds(mut conn: crate::DynoDBPooledConnection) -> DynoResult<()> {
    seed_user(&mut conn)?;

    Ok(())
}
