//! Keyboard key values.


macro_rules! key {
    (
        $(
            $key:ident => $name:expr ,
        )*
    ) => {
        use std::borrow::Cow;
        /// A physical or virtual keyboard key.
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        pub enum Key {
            $(
                $key ,
            )*
            Other(i32)
        }
        
        impl Key {
            /// Returns the name of this key.
            pub fn name(&self) -> Cow<'static, str> {
                match *self {
                    $(
                        self::Key::$key => Cow::Borrowed($name),
                    )*
                    self::Key::Other(val) => format!("Other({})", val).into(),
                }
            }
            
            /// Attempts to find a key with the given name.
            /// All key names are lowercased.
            pub fn from_name(name: &str) -> Option<Key> {
                Some(match name {
                    $(
                        $name => self::Key::$key ,
                    )*
                    _ => return None,
                })
            }
        }
    };
    (
        $(
            $key:ident => $name:expr
        ),*
    ) => {
        key! { $( $key => $name , )* }
    };
}

key! {
    Left    => "left",
    Right   => "right",
    Up      => "up",
    Down    => "down",
    Space   => "space",
    Return  => "return",
    One     => "1",
    Two     => "2",
    Thre    => "3",
    Four    => "4",
}
