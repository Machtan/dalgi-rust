//! Application signals.

/// A simple signal sent by the OS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    /// The user or OS has requested that the application should close.
    QuitRequest,
}
