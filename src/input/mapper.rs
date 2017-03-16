//! Functionality to map from events to input state changes.

use std::collections::HashMap;
use super::description::{InputDesc};
use super::change::{DescribeInputChanges, InputChange};
use super::state::{InputState};

/// Associates abstract input descriptions with game actions, and maps input
/// events to updates to a representation of the game's full action state.
#[derive(Debug, Clone)]
pub struct InputMapper<ActionId: Copy> {
    mappings: HashMap<InputDesc, Vec<ActionId>>,
}

impl<ActionId: Copy> InputMapper<ActionId> {
    /// Creates a new input mapper.
    pub fn new() -> InputMapper<ActionId> {
        InputMapper { mappings: HashMap::new() }
    }
    
    /// Adds a mapping from an input description to a game action.
    pub fn add<D: Into<InputDesc>>(&mut self, action: ActionId, desc: D) {
        self.mappings.entry(desc.into()).or_insert_with(Vec::new).push(action);
    }

    /// Removes all mappings of the given input description.
    pub fn unbind<D: Into<InputDesc>>(&mut self, desc: D) {
        self.mappings.remove(&desc.into());
    }
    
    /// Maps the changes described by the given event to changes to the state.
    pub fn map<E, S>(&self, event: &E, state: &mut S)
        where E: DescribeInputChanges,
              S: InputState<Identifier = ActionId>
    {
        use super::ButtonChange::*;
        use super::InputRef::*;
        event.describe_changes(|change| {
            let action_ids = match self.mappings.get(&change.input()) {
                Some(action_ids) => action_ids,
                None => return,
            };
            
            for action_id in action_ids {
                match (&change, state.get_option(*action_id)) {
                    (&InputChange::Key(_, state), Button(value)) => {
                        match state {
                            Pressed => {
                                value.pressed = true;
                                value.held = true;
                            }
                            Released => {
                                value.released = true;
                                value.held = false;
                            }
                            Repeated => {
                                value.repeats += 1;
                            }
                        }
                    }
                    (&InputChange::Key(_, state), Notification(received)) => {
                        match state {
                            Pressed => {
                                *received = true;
                            }
                            Released | Repeated => {}
                        }
                    }
                    (&InputChange::Notification(_), Notification(received)) => {
                        *received = true;
                    }
                    (_, Button(_)) => {
                        panic!("Button states should only be set by key events");
                    }
                }
            }
        });
    }
}
