//! Functionality to describe input events.

use super::key::Key;
use super::signal::Signal;

/// Which representation of a key should be used (physical vs. virtual).
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Keytype {
    /// The key is a virtual key. This means that key 'a' refers to whatever
    /// button on the keyboard that writes 'a' when pressed.
    Keycode,
    /// The key is a physical key location. This means that key 'a' refers to
    /// the button of the physical keyboard with the label 'a' on it, irregardless
    /// of what is entered when the key is pressed.
    ///
    /// This is mainly useful when using other keys as arrow keys (ie: WASD)
    /// or when relying on the location of the keys, rather than what letter
    /// they start with (for instance using q w and e to toggle transformation modes.)
    Scancode,
}

/// Modifier keys held down while a button is pressed.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Mods(u16); // or whatevs. some bitflag map

/// The description of a button on a keyboard.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct KeyDesc {
    /// The key.
    pub key: Key,
    /// Whether it is a key code (virtual) or scan code (physical) key.
    pub keytype: Keytype,
    /// What modifiers should be pressed.
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

    /// Attempts to parse a key description from the given string.
    ///
    /// All key names are lowercased (invariant from `Key::from_name`)
    ///
    /// # Mini-grammar
    /// - keycode: `a`
    /// - scancode: `[a]` or `[ a ]`
    pub fn parse(mut pattern: &str) -> Option<KeyDesc> {
        pattern = pattern.trim();
        if pattern.starts_with("[") {
            // Scancode
            if let Some(end) = pattern.find("]") {
                let inner = (&pattern[1..end]).trim();
                Key::from_name(inner).map(|k| KeyDesc::new(k).scancode())
            } else {
                // Try to parse as '[' literal
                Key::from_name(pattern).map(KeyDesc::new)
            }
        } else {
            // Keycode
            Key::from_name(pattern).map(KeyDesc::new)
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
    /// The input is a keyboard button (ie: `Jump`).
    Key(KeyDesc),
    /// The input is some sort of signal (ie: `Quit`).
    Signal(Signal),
}
// TODO: Handle modifier checks, somehow (in InputMap?)

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

impl From<Signal> for InputDesc {
    fn from(signal: Signal) -> InputDesc {
        InputDesc::Signal(signal)
    }
}
