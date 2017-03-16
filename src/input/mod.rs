//! Input-related utilities.
//! Includes an input mapping system with optional support for rsdl.

mod key;
mod notification;
mod state;
mod description;
mod change;
mod mapper;

#[macro_use]
mod macros;

#[cfg(feature = "rsdl2-support")]
mod rsdl2_input;

pub use self::key::Key;
pub use self::notification::Notification;
pub use self::state::{InputState, ButtonValue, AdvanceFrame};
pub use self::description::{Keytype, Mods, KeyDesc, InputDesc};
pub use self::change::{DescribeInputChanges, ButtonChange, InputChange};
pub use self::mapper::InputMapper;
