use std::str::FromStr;

use dyno_core::role::Roles;
use dyno_core::{derive_more, serde, AsStr};

#[allow(clippy::upper_case_acronyms)]
#[derive(
    serde::Deserialize,
    serde::Serialize,
    derive_more::Display,
    diesel::AsExpression,
    diesel::FromSqlRow,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
#[serde(crate = "serde")]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct ROLES(pub Roles);

impl ROLES {
    pub fn into_inner(self) -> Roles {
        self.0
    }
    pub fn inner(&self) -> &Roles {
        &self.0
    }
}

impl<B: diesel::backend::Backend> diesel::deserialize::FromSql<diesel::sql_types::Text, B> for ROLES
where
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, B>,
{
    fn from_sql(bytes: B::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let from_sql = <String>::from_sql(bytes)?;
        Roles::from_str(&from_sql).map(ROLES).map_err(Into::into)
    }
}

impl<B: diesel::backend::Backend> diesel::serialize::ToSql<diesel::sql_types::Text, B> for ROLES
where
    str: diesel::serialize::ToSql<diesel::sql_types::Text, B>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, B>,
    ) -> diesel::serialize::Result {
        let Self(inner) = self;
        inner
            .as_str()
            .to_sql(out)
            .map(|_| diesel::serialize::IsNull::No)
            .map_err(Into::into)
    }
}
