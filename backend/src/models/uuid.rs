use diesel::{deserialize, serialize, serialize::Output, sql_types::Text};
use dyno_core::serde;
use dyno_core::uuid::fmt::Simple;
use dyno_core::uuid::Uuid;

#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Clone,
    Copy,
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
pub struct UUID(pub Uuid);

impl UUID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns a reference to the inner of this [`UUID`].
    #[inline]
    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    /// return a owned to the innner of this [`UUID`]
    #[inline]
    pub const fn into_inner(self) -> Uuid {
        self.0
    }

    fn as_str(&self) -> &str {
        unsafe {
            static mut BUFFER: [u8; Simple::LENGTH] = [0x0; Simple::LENGTH];
            self.0.simple().encode_lower(&mut BUFFER)
        }
    }
}

impl Default for UUID {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for UUID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for UUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_simple())
    }
}

impl<B> diesel::deserialize::FromSql<Text, B> for UUID
where
    B: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<Text, B>,
{
    fn from_sql(bytes: B::RawValue<'_>) -> deserialize::Result<Self> {
        String::from_sql(bytes)
            .map(|s| Self(Uuid::try_parse_ascii(s.as_bytes()).expect("this is should not error")))
            .map_err(Into::into)
    }
}

impl<B> diesel::serialize::ToSql<Text, B> for UUID
where
    B: diesel::backend::Backend,
    str: diesel::serialize::ToSql<Text, B>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, B>) -> serialize::Result {
        str::to_sql(self.as_str(), out)
            .map(|_| diesel::serialize::IsNull::No)
            .map_err(Into::into)
    }
}
