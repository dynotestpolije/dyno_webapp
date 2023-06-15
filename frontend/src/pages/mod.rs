mod about;
mod activities;
mod calendar;
mod dashboard;
mod not_found;
mod signin;
mod signup;
mod sop;

pub use about::PageAbout;
pub use activities::PageActivities;
pub use calendar::PageCalendar;
pub use dashboard::PageDashboard;
pub use not_found::PageNotFound;
pub use signin::PageSignIn;
pub use signup::PageSignUp;
pub use sop::PageSop;

pub mod admin;
pub mod setting;
