#![allow(unused)]

use dyno_core::{
    chrono::NaiveDateTime, crypto::TokenDetails, dynotests::DynoTest, serde, users::UserResponse,
    uuid::Uuid, UserSession,
};

use crate::Theme;

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(crate = "serde")]
pub struct Histories {
    pub id: i64,
    pub user_id: i64,
    pub user_uuid: Uuid,
    pub dyno_id: i64,
    pub long_usage: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(crate = "serde")]
pub struct Data {
    dynos: Vec<DynoTest>,
    users: Vec<UserResponse>,
    histories: Vec<Histories>,
}

impl Data {
    pub fn set_dyno(&mut self, dynos: Vec<DynoTest>) {
        self.dynos = dynos;
    }
    pub fn set_users(&mut self, users: Vec<UserResponse>) {
        self.users = users;
    }
    pub fn set_last_usage(&mut self, histories: Vec<Histories>) {
        self.histories = histories;
    }

    pub const fn dyno(&self) -> &Vec<DynoTest> {
        &self.dynos
    }
    pub const fn users(&self) -> &Vec<UserResponse> {
        &self.users
    }
    pub const fn histories(&self) -> &Vec<Histories> {
        &self.histories
    }

    pub fn last_usage(&self) -> Option<&Histories> {
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
    pub const fn token(&self) -> Option<&String> {
        match &self.token_details {
            Some(detail) => detail.token.as_ref(),
            _ => None,
        }
    }
    pub const fn user(&self) -> Option<&UserSession> {
        match &self.token_details {
            Some(detail) => Some(&detail.user),
            None => None,
        }
    }
    pub const fn get_token_details(&self) -> Option<&TokenDetails> {
        self.token_details.as_ref()
    }
    pub fn change_token_details(&mut self, token_details: TokenDetails) {
        core::mem::replace(&mut self.token_details, Some(token_details));
    }
    pub fn delete_token(&mut self) {
        self.token_details = None;
    }
}

impl AppState {
    pub const fn get_data(&self) -> &Data {
        &self.data
    }
}

impl AppState {
    pub const fn theme(&self) -> Theme {
        self.theme
    }
    pub fn swap_theme(&mut self) {
        self.theme = !self.theme;
        match gloo::utils::document_element()
            .set_attribute("data-theme", self.theme.to_str())
            .map_err(|j| j.as_string().unwrap_or(String::new()))
        {
            Ok(()) => (),
            Err(err) => dyno_core::log::error!("{err}"),
        }
    }
}
