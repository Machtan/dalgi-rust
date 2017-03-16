//! Functionality to describe an input state.



// Consider making these u8 flags (pro: size, con: funcall syntax)
// Note: The state stystem can't handle key repeats very well ?

/// The value of a button-type input in a single game frame.
///
/// **Note**: This is slightly lossy, as the number of presses and releases
/// are not counted.
///
/// The button can be both pressed and released in the same frame, so if you
/// need to know if the button is left pressed or not, check the 'held' member.
#[derive(Debug, Clone, PartialEq)]
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
    
    /// Advances the state of the button to the next frame.
    pub fn advance_frame(&mut self) {
        self.pressed = false;
        self.released = false;
        self.repeats = 0;
    }
}

/// Describes the input state of a game.
/// This currently includes the state of all button-type actions.
pub trait InputState {
    /// The type that identifies a button-type input in the state.
    type Identifier: Copy;
    
    /// Returns a reference to the state of a button from its identifier.
    fn get_option<'a>(&'a mut self, id: Self::Identifier) -> InputRef<'a>;
    
    /// Updates the state in preparation of the input in the next game frame.
    /// This ensures that old input changes are cleared.
    fn advance_frame(&mut self);
}

/// The value of an input option.
pub enum InputRef<'a> {
    /// The state of a button.
    Button(&'a mut ButtonValue),
    /// Whether a notification was sent or not.
    Notification(&'a mut bool),
}
