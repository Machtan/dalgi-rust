//! Convenience macros.


/// Creates a new struct to hold the state of the input for the game,
/// along with an enum to access each button state inside it.
///
/// **Note**: The id_enum names match the declared button names, and will
/// therefore usually be lowercase.
///
/// The macro requires the following items in scope:
///
/// `use dalgi::input::{ButtonValue};`
///
/// # Examples
/// ```
/// use dalgi::input::{ButtonValue};
/// 
/// input_state! {
///     Input {
///         jump,
///         shoot,
///         left,
///         right
///     }
///     id_enum: ActionId
/// }
/// ```
#[macro_export]
macro_rules! input_state {
    ( 
        $name:ident { 
            buttons: {
                $($button:ident , )*
            }
            notifications: {
                $($notification:ident , )*
            }
        }
        id_enum: $id_enum:ident;
        mod_name: $mod_name:ident;
    ) => {
        mod $mod_name {
            #[derive(Debug, Clone, PartialEq)]
            pub struct NotificationState {
                $(
                    pub $notification: bool,
                )*
            }
            
            impl NotificationState {
                pub fn new() -> NotificationState {
                    NotificationState {
                        $(
                            $notification: false,
                        )*
                    }
                }
                
                pub fn advance_frame(&mut self) {
                    $(
                        self.$notification = false;
                    )*
                }
            }
        }
        
        /// The input state of a game.
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            notification: $mod_name::NotificationState,
            $(
                pub $button: ButtonValue,
            )*
        }
        
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $id_enum {
            $(
                $button,
            )*
            $(
                $notification,
            )*
        }
        
        impl $name {
            /// Creates a new input state.
            pub fn new() -> Self {
                $name {
                    notification: $mod_name::NotificationState::new(),
                    $(
                        $button: ButtonValue::new(),
                    )*
                }
            }
        }
        
        impl InputState for $name {
            type Identifier = $id_enum;
            
            fn get_option<'a>(&'a mut self, id: $id_enum) -> dalgi::input::InputRef<'a> {
                match id {
                    $(
                        $id_enum::$button => dalgi::input::InputRef::Button(&mut self.$button),
                    )*
                    $(
                        $id_enum::$notification => dalgi::input::InputRef::Notification(&mut self.notification.$notification),
                    )*
                }
            }
            
            fn advance_frame(&mut self) {
                $(
                    self.$button.advance_frame();
                )*
                self.notification.advance_frame();
            }
        }
    }
}