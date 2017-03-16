//! Convenience macros.

/// Generates a set of structs and enums to model an input state.
///
/// For a rough view of what is generated, see `examples/ex_macroless.rs`.
#[macro_export]
macro_rules! input {
    (
        struct $input_type:ident {
            [ button ]
            struct $button_type:ident < $button_id:ident > {
                $(
                    $button:ident ,
                )*
            }

            [ notification ]
            struct $notification_type:ident < $notification_id:ident > {
                $(
                    $notification:ident ,
                )*
            }
        }
    ) => {
        /// The state of a set of button-style inputs [macro-generated].
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $button_type {
            $(
                $button : ButtonValue ,
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
        
        /// The state of a set of notification-style inputs [macro-generated].
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $notification_type {
            $(
                $notification : bool ,
            )*
        }
        
        /// The identifier of a member of the $notification_type struct [macro_generated].
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $notification_id {
            $(
                $notification ,
            )*
        }
        
        /// An input state which can be used with an event mapper [macro_generated].
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $input_type {
            pub button: self::$button_type,
            pub notification: self::$notification_type,
        }
        
        impl $input_type {
            /// Creates a new input state.
            fn new() -> $input_type {
                $input_type::default()
            }
        }
        
        impl dalgi::input::AdvanceFrame for $input_type {
            fn advance_frame(&mut self) {
                $(
                    self.button.$button.advance_frame();
                )*
                $(
                    self.notification.$notification.advance_frame();
                )*
            }
        }
        
        impl dalgi::input::InputState for $input_type {
            type ButtonId = self::$button_id;
            type NotificationId = self::$notification_id;
            
            fn get_button<'a>(&'a mut self, id: &Self::ButtonId) -> &'a mut ButtonValue {
                match *id {
                    $(
                        self::$button_id::$button => &mut self.button.$button ,
                    )*
                }
            }
    
            fn get_notification<'a>(&'a mut self, id: &Self::NotificationId) -> &'a mut bool {
                match *id {
                    $(
                        self::$notification_id::$notification => &mut self.notification.$notification ,
                    )*
                }
            }
        }
    }
}
