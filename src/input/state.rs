//! Functionality to describe an input state.

use std::hash::Hash;

/// An input value that knows how to change its state in the next game frame.
pub trait AdvanceFrame {
    /// Advance this value to the next frame.
    ///
    /// # Examples
    /// ```rust,ignore
    /// impl AdvanceFrame for ButtonValue {
    ///     fn advance_frame(&mut self) {
    ///         // The button has not been pressed in the new frame.
    ///         self.pressed = false;
    ///         // If the button was held, it is still held in the new frame.
    ///         self.held = self.held;
    ///         // The button has not been released in the new frame.
    ///         self.released = false;
    ///         // The button has not been repeated in the new frame.
    ///         self.repeats = 0;
    ///     }
    /// }
    /// ```
    fn advance_frame(&mut self);
}

impl AdvanceFrame for bool {
    fn advance_frame(&mut self) {
        *self = false
    }
}

impl AdvanceFrame for ButtonValue {
    fn advance_frame(&mut self) {
        self.pressed = false;
        self.released = false;
        self.repeats = 0;
    }
}

/// Bounds for a type that can be used to identify inputs in an input state.
pub trait InputIndex: PartialEq + Eq + Hash + Copy {}

impl<T> InputIndex for T where T: PartialEq + Eq + Hash + Copy {}

pub trait InputState: AdvanceFrame {
    /// Identifies a button-style input.
    type ButtonId: InputIndex;

    /// Identifies a notification-style input.
    type NotificationId: InputIndex;

    /// Returns the state of the button.
    fn get_button<'a>(&'a mut self, id: &Self::ButtonId) -> &'a mut ButtonValue;

    /// Returns the state of the notification.
    fn get_notification<'a>(&'a mut self, id: &Self::NotificationId) -> &'a mut bool;
}

/// The value of a button-type input in a single game frame.
///
/// **Note**: This is slightly lossy, as the number of presses and releases
/// are not counted.
///
/// The button can be both pressed and released in the same frame, so if you
/// need to know if the button is left pressed or not, check the 'held' member.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ButtonValue {
    pub pressed: bool,
    pub held: bool,
    pub released: bool,
    pub repeats: u8,
}

impl ButtonValue {
    /// Creates a new button value description.
    pub fn new() -> ButtonValue {
        ButtonValue {
            pressed: false,
            released: false,
            held: false,
            repeats: 0,
        }
    }
}
