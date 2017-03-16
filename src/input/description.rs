//! Functionality to describe input events.

use super::key::Key;
use super::notification::Notification;

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
    Notification(Notification),
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

impl From<Notification> for InputDesc {
    fn from(notification: Notification) -> InputDesc {
        InputDesc::Notification(notification)
    }
}