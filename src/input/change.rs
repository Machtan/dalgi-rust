//! Functionality to describe changes to an input state.
use super::description::{KeyDesc, InputDesc};
use super::signal::Signal;

/// The state of a button.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ButtonChange {
    /// The button is has just been pressed.
    Pressed,
    /// The button has just been released.
    Released,
    /// The button is held, and the input manager is firing a 'repeat' event.
    Repeated,
}

/// The representation of a change to the input state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputChange {
    /// A keyboard button was changed.
    Key(KeyDesc, ButtonChange),
    /// A signal was sent.
    Signal(Signal),
}

impl InputChange {
    /// Returns the input that this change pertains to.
    /// Ex: the button with the virtual key 'a', if this is about a change
    /// to this button (the button is pressed, released, etc.).
    pub fn input(&self) -> InputDesc {
        use self::InputChange::*;
        match *self {
            Key(desc, _) => InputDesc::Key(desc),
            Signal(signal) => InputDesc::Signal(signal),
        }
    }
}

impl From<Signal> for InputChange {
    fn from(signal: Signal) -> InputChange {
        InputChange::Signal(signal)
    }
}

/// A trait to let an 'input event' describe which changes it contains.
pub trait DescribeInputChanges {
    /// Tells the handler which changes this object represents.
    ///
    /// **Note**: If the changes cannot be described by dalgi, just ignore the handler.
    fn describe_changes<F: FnMut(InputChange)>(&self, handler: F);
}

impl DescribeInputChanges for InputChange {
    fn describe_changes<F: FnMut(InputChange)>(&self, mut handler: F) {
        handler(self.clone())
    }
}
