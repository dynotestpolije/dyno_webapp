mod activities;
mod dashboard;
mod live;
mod not_found;
mod setting_profile;
mod signin;
mod signup;
mod sop;

pub use activities::PageActivities;
pub use dashboard::PageDashboard;
pub use live::PageLive;
pub use not_found::PageNotFound;
pub use setting_profile::PageSettingProfile;
pub use signin::PageSignIn;
pub use signup::PageSignUp;
pub use sop::PageSop;

pub mod admin;
