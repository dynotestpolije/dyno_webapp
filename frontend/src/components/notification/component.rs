#![allow(unused)]

use super::{
    classes, format_date_time, function_component, html, Callback, Classes, Duration, Html, Local,
    MouseEvent, NaiveDateTime, Notifiable, NotifiableComponentFactory, Properties, Uuid,
};

/// Standard notification type.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum NotificationType {
    /// Represents some informative message for the user.
    #[default]
    Info,

    /// Represents some warning.
    Warn,

    /// Represents some error message.
    Error,

    /// Custom notification type.
    ///
    /// You can use this option when you want to set the custom style of your notification
    /// but don't want to write an entire custom component from scratch.
    Custom(Classes),
}

impl From<&str> for NotificationType {
    fn from(data: &str) -> Self {
        match data {
            "info" => Self::Info,
            "warn" => Self::Warn,
            "error" => Self::Error,
            data => Self::Custom(data.to_owned().into()),
        }
    }
}

impl From<&NotificationType> for Classes {
    fn from(notification_type: &NotificationType) -> Self {
        match notification_type {
            NotificationType::Info => classes!("info"),
            NotificationType::Warn => classes!("warn"),
            NotificationType::Error => classes!("error"),
            NotificationType::Custom(classes) => classes.clone(),
        }
    }
}

/// Standard notification.
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub(super) id: Uuid,
    pub(super) notification_type: NotificationType,
    pub(super) title: Option<String>,
    pub(super) text: String,

    pub(super) spawn_time: NaiveDateTime,
    pub(super) lifetime: Duration,
    pub(super) full_lifetime: Duration,
    pub(super) paused: bool,
}

impl Notification {
    const NOTIFICATION_LIFETIME: Duration = Duration::from_secs(3);
    /// Creates a new standard notification from notification type, title, text, and lifetime duration.
    pub fn new(
        notification_type: NotificationType,
        title: impl Into<String>,
        text: impl Into<String>,
        lifetime: Duration,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_type,
            title: Some(title.into()),
            text: text.into(),

            spawn_time: Local::now().naive_local(),
            lifetime,
            full_lifetime: lifetime,
            paused: false,
        }
    }

    /// Creates a new standard notification from notification type and text.
    ///
    /// Title will be omitted. Notification lifetime is equal to the [`Self::NOTIFICATION_LIFETIME`].
    pub fn from_description_and_type(
        notification_type: NotificationType,
        text: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_type,
            title: None,
            text: text.into(),

            spawn_time: Local::now().naive_local(),
            lifetime: Self::NOTIFICATION_LIFETIME,
            full_lifetime: Self::NOTIFICATION_LIFETIME,
            paused: false,
        }
    }

    /// Set the title for the notification.
    pub fn with_title(self, new_title: impl Into<String>) -> Self {
        let Notification {
            id,
            notification_type,
            title: _,
            text: description,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title: Some(new_title.into()),
            text: description,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        }
    }

    /// Set the type for the notification.
    pub fn with_type(self, new_notification_type: NotificationType) -> Self {
        let Notification {
            id,
            notification_type: _,
            title,
            text: description,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type: new_notification_type,
            title,
            text: description,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        }
    }

    /// Set the text for the notification.
    pub fn with_text(self, new_text: impl Into<String>) -> Self {
        let Notification {
            id,
            notification_type,
            title,
            text: _,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title,
            text: new_text.into(),

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        }
    }

    /// Resets notification lifetime.
    ///
    /// It means that after this method invocation, the lifetime of the notification will be equal to the start value.
    pub fn reset_lifetime(self) -> Self {
        let Notification {
            id,
            notification_type,
            title,
            text,

            spawn_time,
            lifetime: _,
            full_lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title,
            text,

            spawn_time,
            lifetime: full_lifetime,
            full_lifetime,
            paused,
        }
    }
}

impl Notifiable for Notification {
    fn id(&self) -> Uuid {
        self.id
    }

    fn apply_tick(&mut self, time: Duration) {
        self.lifetime = self.lifetime.checked_sub(time).unwrap_or(Duration::ZERO);
    }

    fn is_alive(&self) -> bool {
        self.lifetime.is_zero()
    }

    fn mouse_in(&mut self) {
        self.paused = true;
    }

    fn mouse_out(&mut self) {
        self.paused = false;
        self.lifetime = self.full_lifetime;
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}

/// Props for [`NotificationComponent`]
#[derive(Properties, Clone, PartialEq)]
pub struct NotificationComponentProps {
    /// Notification object to render.
    pub notification: Notification,

    /// *onclick* event callback.
    pub onclick: Callback<MouseEvent>,

    /// *onenter* event callback.
    pub onenter: Callback<MouseEvent>,

    /// *onleave* event callback.
    pub onleave: Callback<MouseEvent>,
}

/// Standard notification component.
#[function_component(NotificationComponent)]
pub fn notification_component(props: &NotificationComponentProps) -> Html {
    let title = props.notification.title.as_ref();
    let text = &props.notification.text;
    let notification_type = &props.notification.notification_type;
    let spawn_time = &props.notification.spawn_time;

    let onclick = props.onclick.clone();
    let onenter = props.onenter.clone();
    let onleave = props.onleave.clone();

    let mut classes = vec![classes!("notification"), notification_type.into()];
    if props.notification.is_paused() {
        classes.push(classes!("paused"));
    }

    html! {
        <div {onclick} onmouseenter={onenter} onmouseleave={onleave} class={classes}>
            {if let Some(title) = title {
                html! { <span class={classes!("notification-title")}>{title}</span> }
            } else {
                html! {}
            }}
            <span>{text}</span>
            <span class={classes!("time")}>{format_date_time(spawn_time)}</span>
        </div>
    }
}

/// Standard notification factory.
///
/// This factory used for [`Notification`] components creation.
#[derive(Clone, PartialEq, Default)]
pub struct NotificationFactory;

impl NotifiableComponentFactory<Notification> for NotificationFactory {
    fn component(
        &self,
        notification: Notification,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <NotificationComponent {notification} {onclick} {onenter} {onleave} />
        }
    }
}
