//! Functionality to map from events to input state changes.

use std::collections::HashMap;
use super::key::Key;
use super::notification::Notification;
use super::description::{InputDesc, KeyDesc};
use super::change::{DescribeInputChanges, InputChange};
use super::state::InputState;

/// A description of events that can change the state of a button-type input.
pub enum ButtonUpdateSource {
    Key(KeyDesc),
}

impl From<KeyDesc> for ButtonUpdateSource {
    fn from(keydesc: KeyDesc) -> ButtonUpdateSource {
        ButtonUpdateSource::Key(keydesc)
    }
}

impl From<Key> for ButtonUpdateSource {
    fn from(key: Key) -> ButtonUpdateSource {
        ButtonUpdateSource::Key(KeyDesc::new(key))
    }
}

impl Into<InputDesc> for ButtonUpdateSource {
    fn into(self: ButtonUpdateSource) -> InputDesc {
        match self {
            ButtonUpdateSource::Key(desc) => InputDesc::Key(desc),
        }
    }
}

/// A description of events that can change the state of a notification-type input.
pub enum NotificationUpdateSource {
    Notification(Notification),
    Key(KeyDesc),
}

impl From<KeyDesc> for NotificationUpdateSource {
    fn from(keydesc: KeyDesc) -> NotificationUpdateSource {
        NotificationUpdateSource::Key(keydesc)
    }
}

impl From<Key> for NotificationUpdateSource {
    fn from(key: Key) -> NotificationUpdateSource {
        NotificationUpdateSource::Key(KeyDesc::new(key))
    }
}

impl From<Notification> for NotificationUpdateSource {
    fn from(notification: Notification) -> NotificationUpdateSource {
        NotificationUpdateSource::Notification(notification)
    }
}

impl Into<InputDesc> for NotificationUpdateSource {
    fn into(self: NotificationUpdateSource) -> InputDesc {
        match self {
            NotificationUpdateSource::Key(desc) => InputDesc::Key(desc),
            NotificationUpdateSource::Notification(note) => InputDesc::Notification(note),
        }
    }
}

/// Associates abstract input descriptions with game actions, and maps input
/// events to updates to a representation of the game's full action state.
#[derive(Debug, Clone)]
pub struct InputMapper<BI, NI> {
    buttons: HashMap<InputDesc, Vec<BI>>,
    notifications: HashMap<InputDesc, Vec<NI>>,
}

impl<BI, NI> InputMapper<BI, NI> {
    /// Creates a new input mapper.
    pub fn new() -> InputMapper<BI, NI> {
        InputMapper {
            buttons: HashMap::new(),
            notifications: HashMap::new(),
        }
    }

    /// Adds a mapping from a button input source to a button action.
    pub fn add_button<D: Into<ButtonUpdateSource>>(&mut self, action: BI, desc: D) {
        self.buttons.entry(desc.into().into()).or_insert_with(Vec::new).push(action);
    }

    /// Adds a mapping from a notification input source to a notification action.
    pub fn add_notification<D: Into<NotificationUpdateSource>>(&mut self, action: NI, desc: D) {
        self.notifications.entry(desc.into().into()).or_insert_with(Vec::new).push(action);
    }

    /// Applies the changes described by the given event to the input state.
    pub fn map<E, S>(&self, event: &E, state: &mut S)
        where E: DescribeInputChanges,
              S: InputState<ButtonId = BI, NotificationId = NI>
    {
        use super::ButtonChange::*;
        event.describe_changes(|change| {
            let input = change.input();
            // BUTTON MAPPING
            for button_id in self.buttons.get(&input).into_iter().flat_map(|a| a) {
                let mut button = state.get_button(&button_id);
                match change {
                    InputChange::Key(_, state) => {
                        match state {
                            Pressed => {
                                button.pressed = true;
                                button.held = true;
                            }
                            Released => {
                                button.released = true;
                                button.held = false;
                            }
                            Repeated => {
                                button.repeats += 1;
                            }
                        }
                    }
                    InputChange::Notification(_) => unreachable!(),
                }
            }

            // NOTIFICATION MAPPING
            for notification_id in self.notifications.get(&input).into_iter().flat_map(|a| a) {
                let mut notification_received = state.get_notification(&notification_id);
                match change {
                    InputChange::Key(_, state) => {
                        match state {
                            Pressed => {
                                *notification_received = true;
                            }
                            Released | Repeated => {}
                        }
                    }
                    InputChange::Notification(_) => {
                        *notification_received = true;
                    }
                }
            }
        });
    }
}
