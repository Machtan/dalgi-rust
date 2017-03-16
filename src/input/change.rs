//! Functionality to describe changes to an input state.
use super::description::{KeyDesc, InputDesc};
use super::notification::Notification;

/// The state of a button.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ButtonChange {
    Pressed,
    Released,
    Repeated,
}

/// The representation of a change to the input state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputChange {
    Key(KeyDesc, ButtonChange),
    Notification(Notification),
}

impl InputChange {
    /// Returns the input that this change pertains to.
    /// Ex: the button with the virtual key 'a', if this is about a change
    /// to this button (the button is pressed, released, etc.).
    pub fn input(&self) -> InputDesc {
        use self::InputChange::*;
        match *self {
            Key(desc, _) => InputDesc::Key(desc),
            Notification(notification) => InputDesc::Notification(notification),
        }
    }
}

impl From<Notification> for InputChange {
    fn from(notification: Notification) -> InputChange {
        InputChange::Notification(notification)
    }
}

/// A trait to let an 'input event' describe which changes it contains.
pub trait DescribeInputChanges {
    /// Tells the handler which changes this object represents.
    ///
    /// **Note**: If the changes cannot be described, just don't call the handler.
    fn describe_changes<F: FnMut(InputChange)>(&self, handler: F);
}

impl DescribeInputChanges for InputChange {
    fn describe_changes<F: FnMut(InputChange)>(&self, mut handler: F) {
        handler(self.clone())
    }
}
