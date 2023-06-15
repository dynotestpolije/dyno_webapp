mod component;
mod manager;
mod provider;

use std::any::Any;

pub use component::{
    Notification, NotificationComponent, NotificationComponentProps, NotificationFactory,
    NotificationType,
};
pub use manager::NotificationsManager;
pub use provider::{NotificationsPosition, NotificationsProvider, NotificationsProviderProps};

use dyno_core::{
    chrono::{Local, NaiveDateTime},
    uuid::Uuid,
};
use std::time::Duration;
use yew::{html::Scope, prelude::*};

/// Every notification has such thing as *lifetime*.
/// This is simply the amount of time that this notification is still "alive" (which means is present on the screen or is visible).
/// Methods like [`Notifiable::apply_tick`], [`Notifiable::is_alive`], etc, are used by library internal to control life of the notification.
pub trait Notifiable: Any {
    /// Returns the id of the notification. Every notification has the id of the Uuid type and it should be unique.
    fn id(&self) -> Uuid;

    /// Applies some amount of time to this notification.
    ///
    /// # Arguments
    ///
    /// * `time` - An amount of time that has been spent.
    fn apply_tick(&mut self, time: Duration);

    /// Check if the notification is still "alive".
    /// If it returns false, then this notification will be deleted (disappeared) on the next time tick.
    fn is_alive(&self) -> bool;

    /// Check if the notification is still "paused".
    /// It means, that when notification is paused (this function returns true)
    /// then time does not affect the notification lifetime.
    fn is_paused(&self) -> bool;

    /// This function calls when the mouse enters this notification.
    fn mouse_in(&mut self);

    /// This function calls when the mouse leaves this notification.
    fn mouse_out(&mut self);
}

/// This trait provides an interface for the notification component factory.
///
/// This trait implementors are used be `yew-notifications` for notification components rendering.
pub trait NotifiableComponentFactory<T: Notifiable> {
    /// Creates a new notification component that can be rendered in `yew`.
    fn component(
        &self,
        notification: T,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html;
}
pub trait LinkNotification {
    fn notif(&self, notification: Notification);
}

impl<COMP: Component> LinkNotification for Scope<COMP> {
    fn notif(&self, notification: Notification) {
        self.context::<NotificationsManager<Notification>>(Callback::noop())
            .map(|(ctx, _)| ctx.spawn(notification))
            .unwrap_or_default()
    }
}

#[hook]
pub fn use_notification<T: Notifiable + PartialEq + Clone>() -> manager::NotificationsManager<T> {
    use_context::<manager::NotificationsManager<T>>().unwrap_or_default()
}

pub fn format_date_time(datetime: &NaiveDateTime) -> String {
    datetime.format("%r %v").to_string()
}

// notification_type: NotificationType,
// title: impl Into<String>,
// text: impl Into<String>,
// lifetime: Duration,
#[macro_export]
macro_rules! notif_macros {
    ($t:ident, $title:expr, $($text:tt)*) => {
        $crate::components::notification::Notification::new(
            $crate::components::notification::NotificationType::$t,
            $title,
            format!($($text)*),
            std::time::Duration::from_secs(4)
        )
    };
}

#[macro_export]
macro_rules! notif_info {
    ($($args:tt)*) => ($crate::notif_macros!( Info, $($args)*));
}
#[macro_export]
macro_rules! notif_warn {
    ($($args:tt)*) => ($crate::notif_macros!( Warn, $($args)*));
}

#[macro_export]
macro_rules! notif_error {
    ($($args:tt)*) => ($crate::notif_macros!( Error, $($args)*));
}
