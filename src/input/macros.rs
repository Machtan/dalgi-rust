//! Convenience macros.

/// Generates a set of structs and enums to model an input state.
///
/// For a rough view of what is generated, see `examples/ex_macroless.rs`.
#[macro_export]
macro_rules! input {
    (
        pub struct $input_type:ident {
            [ button ]
            pub struct $button_type:ident < $button_id:ident > {
                $(
                    $button:ident ,
                )*
            }

            [ signal ]
            pub struct $signal_type:ident < $signal_id:ident > {
                $(
                    $signal:ident ,
                )*
            }
        }
    ) => {
        /// The state of a set of button-style inputs [macro-generated].
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $button_type {
            $(
                pub $button : dalgi::input::ButtonValue ,
            )*
        }
        
        /// The identifier of a member of the $button_type struct [macro_generated].
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $button_id {
            $(
                $button ,
            )*
        }
        
        /// The state of a set of signal-style inputs [macro-generated].
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $signal_type {
            $(
                pub $signal : bool ,
            )*
        }
        
        /// The identifier of a member of the $signal_type struct [macro_generated].
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $signal_id {
            $(
                $signal ,
            )*
        }
        
        /// An input state which can be used with an event map [macro_generated].
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $input_type {
            pub button: self::$button_type,
            pub signal: self::$signal_type,
        }
        
        impl $input_type {
            /// Creates a new input state.
            pub fn new() -> $input_type {
                $input_type::default()
            }
        }
        
        impl dalgi::input::AdvanceFrame for $input_type {
            fn advance_frame(&mut self) {
                $(
                    self.button.$button.advance_frame();
                )*
                $(
                    self.signal.$signal.advance_frame();
                )*
            }
        }
        
        impl dalgi::input::InputState for $input_type {
            type ButtonId = self::$button_id;
            type SignalId = self::$signal_id;
            
            fn get_button<'a>(&'a mut self, id: &Self::ButtonId) -> &'a mut dalgi::input::ButtonValue {
                match *id {
                    $(
                        self::$button_id::$button => &mut self.button.$button ,
                    )*
                }
            }
    
            fn get_signal<'a>(&'a mut self, id: &Self::SignalId) -> &'a mut bool {
                match *id {
                    $(
                        self::$signal_id::$signal => &mut self.signal.$signal ,
                    )*
                }
            }
        }
    }
}
