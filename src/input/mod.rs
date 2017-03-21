//! Input-related utilities.
//! Includes an input mapping system with optional support for rsdl.

mod key;
mod signal;
mod state;
mod description;
mod change;
mod map;

#[macro_use]
mod macros;

#[cfg(feature = "rsdl2-support")]
mod rsdl2_input;

pub use self::key::Key;
pub use self::signal::Signal;
pub use self::state::{InputState, ButtonValue, AdvanceFrame};
pub use self::description::{Keytype, Mods, KeyDesc, InputDesc};
pub use self::change::{DescribeInputChanges, ButtonChange, InputChange};
pub use self::map::InputMap;
