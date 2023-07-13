use dyno_core::{
    chrono::{DateTime, Utc},
    serde, DynoConfig, UserSession,
};

pub mod dyno;
pub mod history;
pub mod info;
pub mod role;
pub mod user;
pub mod uuid;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(crate = "serde")]
pub struct ActiveUser {
    pub user: Option<UserSession>,
    pub dyno: Option<DynoConfig>,
    pub start: DateTime<Utc>,
}

impl Default for ActiveUser {
    fn default() -> Self {
        Self::new()
    }
}

impl ActiveUser {
    pub fn new() -> Self {
        Self {
            user: None,
            dyno: None,
            start: Utc::now(),
        }
    }
    pub fn to_history(&self) -> Option<history::NewHistory> {
        let Some(user) = self.user else { return None; };
        Some(history::NewHistory::new(user.id))
    }
    pub fn set_user(mut self, user: UserSession) -> Self {
        self.user = Some(user);
        self
    }
    pub fn set_dyno(mut self, dyno: DynoConfig) -> Self {
        self.dyno = Some(dyno);
        self
    }
}
