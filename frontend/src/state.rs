#![allow(unused)]

use dyno_core::{
    chrono::NaiveDateTime, crypto::TokenDetails, dynotests::DynoTest, serde, UserSession,
};

use crate::Theme;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(crate = "serde")]
pub struct Data {
    dyno: Vec<DynoTest>,
    last_usage: NaiveDateTime,
}
impl Data {
    pub const fn dyno(&self) -> &Vec<DynoTest> {
        &self.dyno
    }
    pub const fn last_usage(&self) -> NaiveDateTime {
        self.last_usage
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, yewdux::store::Store,
)]
#[serde(crate = "serde")]
#[store(storage = "local")]
pub struct AppState {
    token_details: Option<TokenDetails>,
    data: Data,
    theme: Theme,
}

impl AppState {
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
        self.token_details.take();
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
