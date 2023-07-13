#![allow(unused)]

use dyno_core::{
    chrono::NaiveDateTime, crypto::TokenDetails, dynotests::DynoTest, serde, users::UserResponse,
    uuid::Uuid, DynoConfig, HistoryResponse, PlotColor, UserSession,
};

use crate::Theme;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(crate = "serde")]
pub struct Data {
    dynos: Vec<DynoTest>,
    users: Vec<UserResponse>,
    infos: Vec<DynoConfig>,
    histories: Vec<HistoryResponse>,
}

impl Data {
    pub fn set_dyno(&mut self, dynos: Vec<DynoTest>) {
        self.dynos = dynos;
    }
    pub fn set_users(&mut self, users: Vec<UserResponse>) {
        self.users = users;
    }
    pub fn set_infos(&mut self, infos: Vec<DynoConfig>) {
        self.infos = infos;
    }
    pub fn set_last_usage(&mut self, histories: Vec<HistoryResponse>) {
        self.histories = histories;
    }

    pub const fn dyno(&self) -> &Vec<DynoTest> {
        &self.dynos
    }
    pub const fn users(&self) -> &Vec<UserResponse> {
        &self.users
    }
    pub const fn histories(&self) -> &Vec<HistoryResponse> {
        &self.histories
    }
    pub const fn infos(&self) -> &Vec<DynoConfig> {
        &self.infos
    }

    pub fn last_usage(&self) -> Option<&HistoryResponse> {
        self.histories.last()
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, yewdux::store::Store,
)]
#[serde(crate = "serde")]
#[store(storage = "local")]
pub struct AppState {
    me: Option<UserResponse>,
    token_details: Option<TokenDetails>,
    data: Data,
    theme: Theme,
}

impl AppState {
    pub const fn me(&self) -> Option<&UserResponse> {
        match &self.me {
            Some(me) => Some(me),
            None => None,
        }
    }
    pub fn set_me(&mut self, me: Option<UserResponse>) {
        self.me = me;
    }
    pub const fn token_session(&self) -> Option<&String> {
        match &self.token_details {
            Some(detail) => detail.token.as_ref(),
            _ => None,
        }
    }
    pub const fn user_session(&self) -> Option<&UserSession> {
        match &self.token_details {
            Some(detail) => Some(&detail.user),
            None => None,
        }
    }
    pub const fn get_token_details(&self) -> Option<&TokenDetails> {
        self.token_details.as_ref()
    }
    pub fn set_token_details(&mut self, token_details: TokenDetails) {
        self.token_details = Some(token_details);
    }
    pub fn delete_token(&mut self) {
        self.token_details = None;
    }
}

impl AppState {
    pub const fn get_data(&self) -> &Data {
        &self.data
    }
    pub fn get_data_mut(&mut self) -> &mut Data {
        &mut self.data
    }
}

impl AppState {
    pub const fn plot_color(&self) -> PlotColor {
        match self.theme {
            Theme::Dark => PlotColor::dark(),
            Theme::Light => PlotColor::light(),
        }
    }
    pub const fn theme(&self) -> Theme {
        self.theme
    }
    pub fn swap_theme(&mut self) {
        self.theme = !self.theme;
    }
}
