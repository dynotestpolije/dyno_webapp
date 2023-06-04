use diesel::{deserialize, serialize, serialize::Output, sql_types::Text};
use dyno_core::uuid::Uuid;
use dyno_core::{serde, DynoErr, DynoResult};

#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    diesel::FromSqlRow,
    diesel::AsExpression,
    serde::Serialize,
    serde::Deserialize,
    Hash,
    Eq,
    PartialEq,
)]
#[serde(crate = "serde")]
#[diesel(sql_type = Text)]
pub struct UUID(pub String);

impl UUID {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Returns a reference to the inner of this [`UUID`].
    #[inline]
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// return a owned to the innner of this [`UUID`]
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Returns the uuid of this [`UUID`].
    ///
    /// # Errors
    ///
    /// This function will return an error if [`uuid::Uuid::parse_str`] failed ot parse from str.
    #[inline]
    pub fn uuid(&self) -> DynoResult<Uuid> {
        Uuid::parse_str(&self.0).map_err(DynoErr::parsing_error)
    }
}

impl Default for UUID {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<UUID> for Uuid {
    type Error = DynoErr;
    fn try_from(value: UUID) -> Result<Self, Self::Error> {
        value.uuid()
    }
}

impl std::fmt::Display for UUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<B> diesel::deserialize::FromSql<Text, B> for UUID
where
    B: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<Text, B>,
{
    fn from_sql(bytes: B::RawValue<'_>) -> deserialize::Result<Self> {
        String::from_sql(bytes).map(UUID).map_err(Into::into)
    }
}

impl<B> diesel::serialize::ToSql<Text, B> for UUID
where
    B: diesel::backend::Backend,
    str: diesel::serialize::ToSql<Text, B>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, B>) -> serialize::Result {
        str::to_sql(&self.0, out)
            .map(|_| diesel::serialize::IsNull::No)
            .map_err(Into::into)
    }
}
