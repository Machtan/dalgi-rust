//! Keyboard key values.

// The 'key' prefix is to allow doccomments (otherwise it has ambiguity since
// it apparently thinks attributes can be identifiers too)
macro_rules! key {
    (
        $(
            $(#[$attr:meta])*
            key $key:ident => $name:expr ,
        )*
    ) => {
        use std::borrow::Cow;
        /// A physical or virtual keyboard key.
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        pub enum Key {
            $(
                $( #[$attr] )*
                $key ,
            )*
            /// A key that isn't in dalgi, but can be identified by a number.
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
            $(#[$attr:meta])*
            key $key:ident => $name:expr
        ),*
    ) => {
        key! { 
            $(  
                $( $attr )*
                key $key => $name , 
            )*
        }
    };
}

key! {
    /// The left arrow.
    key Left    => "left",
    /// The right arrow.
    key Right   => "right",
    /// The up arrow.
    key Up      => "up",
    /// The left arrow.
    key Down    => "down",
    /// The space bar.
    key Space   => "space",
    /// The `return` or `enter` button.
    key Return  => "return",
    /// The number 1.
    key One     => "1",
    /// The number 2.
    key Two     => "2",
    /// The number 3.
    key Three   => "3",
    /// The number 4.
    key Four    => "4",
}
