use std::collections::HashMap;
use std::marker::Copy;
pub use input_key::Key;


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
    fn get_option(&mut self, id: Self::Identifier) -> &mut ButtonValue;
    
    /// Updates the state in preparation of the input in the next game frame.
    /// This ensures that old input changes are cleared.
    fn advance_frame(&mut self);
}

#[macro_export]
/// Creates a new struct to hold the state of the input for the game,
/// along with an enum to access each button state inside it.
///
/// **Note**: The id_enum names match the declared button names, and will
/// therefore usually be lowercase.
///
/// The macro requires the following items in scope:
///
/// `use dalgi::input::{ButtonValue};`
///
/// # Examples
/// ```
/// use dalgi::input::{ButtonValue};
/// 
/// input_state! {
///     Input {
///         jump,
///         shoot,
///         left,
///         right
///     }
///     id_enum: ActionId
/// }
/// ```
macro_rules! input_state {
    ( $name:ident { $($input:ident),* } id_enum: $id_enum:ident ) => {
        /// The input state of a game.
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            $(
                pub $input: ButtonValue,
            )*
        }
        
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $id_enum {
            $(
                $input,
            )*
        }
        
        impl $name {
            /// Creates a new input state.
            pub fn new() -> Self {
                $name {
                    $(
                        $input: ButtonValue::new(),
                    )*
                }
            }
        }
        
        impl InputState for $name {
            type Identifier = $id_enum;
            
            fn get_option(&mut self, id: $id_enum) -> &mut ButtonValue {
                match id {
                    $(
                        $id_enum::$input => &mut self.$input,
                    )*
                }
            }
            
            fn advance_frame(&mut self) {
                $(
                    self.$input.advance_frame();
                )*
            }
        }
    }
}

/// The state of a button.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ButtonChange {
    Pressed,
    Released,
    Repeated,
}

/// Which representation of a key should be used (physical vs. virtual).
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Keytype {
    Keycode,
    Scancode,
}

/// Modifier keys held down while a button is pressed.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Mods(u16); // or whatevs. some bitflag map

/// The description of a button on a keyboard.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct KeyDesc {
    pub key: Key,
    pub keytype: Keytype,
    pub mods: Mods,
}

impl KeyDesc {
    /// Creates a new key description with the given key.
    pub fn new(key: Key) -> KeyDesc {
        KeyDesc {
            key: key,
            keytype: Keytype::Keycode,
            mods: Mods(0),
        }
    }

    /// Builder method to mark this input to use the scancode rather than
    /// key code. 
    /// The scancode is the physical keyboard location of the button, whereas
    /// the key code is what this button is mapped to inside the OS.
    pub fn scancode(mut self) -> KeyDesc {
        self.keytype = Keytype::Scancode;
        self
    }

    /// Builder method to add a set of modifier keys to the description
    pub fn mods(mut self, mods: Mods) -> KeyDesc {
        self.mods = mods;
        self
    }
}

impl From<Key> for KeyDesc {
    fn from(value: Key) -> KeyDesc {
        KeyDesc::new(value)
    }
}

/// The description of a physical game input.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum InputDesc {
    Key(KeyDesc),
}
// TODO: Handle modifier checks, somehow (in InputMapper?)

impl From<KeyDesc> for InputDesc {
    fn from(keydesc: KeyDesc) -> InputDesc {
        InputDesc::Key(keydesc)
    }
}

impl From<Key> for InputDesc {
    fn from(key: Key) -> InputDesc {
        InputDesc::Key(KeyDesc::new(key))
    }
}

/// The representation of a change to the input state.
pub enum InputChange {
    Key(KeyDesc, ButtonChange),
}

impl InputChange {
    /// Returns the input that this change pertains to.
    /// Ex: the button with the virtual key 'a', if this is about a change
    /// to this button (the button is pressed, released, etc.).
    pub fn input(&self) -> InputDesc {
        use self::InputChange::*;
        match *self {
            Key(desc, _) => InputDesc::Key(desc),
        }
    }
}

/// Associates abstract input descriptions with game actions, and maps input
/// events to updates to a representation of the game's full action state.
#[derive(Debug, Clone)]
pub struct InputMapper<ActionId: Copy> {
    mappings: HashMap<InputDesc, Vec<ActionId>>,
}

/// A trait to let an 'input event' describe which changes it contains.
pub trait DescribeInputChange {
    /// Tells the handler which changes this object represents.
    ///
    /// **Note**: If the changes cannot be described, just don't call the handler.
    fn describe_changes<F: FnMut(InputChange)>(&self, handler: F);
}

impl<ActionId: Copy> InputMapper<ActionId> {
    /// Creates a new input mapper.
    pub fn new() -> InputMapper<ActionId> {
        InputMapper { mappings: HashMap::new() }
    }
    
    /// Adds a mapping from an input description to a game action.
    pub fn add<D: Into<InputDesc>>(&mut self, action: ActionId, desc: D) {
        self.mappings.entry(desc.into()).or_insert_with(Vec::new).push(action);
    }

    /// Removes all mappings of the given input description.
    pub fn unbind<D: Into<InputDesc>>(&mut self, desc: D) {
        self.mappings.remove(&desc.into());
    }
    
    /// Maps the changes described by the given event to changes to the state.
    pub fn map<E, S>(&self, event: &E, state: &mut S)
        where E: DescribeInputChange,
              S: InputState<Identifier = ActionId>
    {
        use self::ButtonChange::*;
        event.describe_changes(|change| {
            let action_ids = match self.mappings.get(&change.input()) {
                Some(action_ids) => action_ids,
                None => return,
            };
            
            for action_id in action_ids {
                let mut value = state.get_option(*action_id);
                match change {
                    InputChange::Key(_, state) => {
                        match state {
                            Pressed => {
                                value.pressed = true;
                                value.held = true;
                            }
                            Released => {
                                value.released = true;
                                value.held = false;
                            }
                            Repeated => {
                                value.repeats += 1;
                            }
                        }
                    }
                }
            }
        });
    }
}
