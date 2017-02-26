
/// A physical or virtual keyboard key.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
    Other(i32),
}
