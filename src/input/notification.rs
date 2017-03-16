//! Application notifications.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Notification {
    /// The user or OS has requested that the application should close.
    QuitRequest
}
