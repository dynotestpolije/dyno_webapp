mod activities;
mod dashboard;
mod not_found;
mod signin;
mod signup;
mod sop;
mod setting_profile;

pub use activities::PageActivities;
pub use dashboard::PageDashboard;
pub use not_found::PageNotFound;
pub use signin::PageSignIn;
pub use signup::PageSignUp;
pub use sop::PageSop;
pub use setting_profile::PageSettingProfile;

pub mod admin;
