//! Functionality to map from events to input state changes.

use std::collections::{HashMap, HashSet};
use super::key::Key;
use super::signal::Signal;
use super::description::{InputDesc, KeyDesc};
use super::change::{DescribeInputChanges, InputChange};
use super::state::{InputState, InputIndex};

/// A description of events that can change the state of a button-type input.
pub enum ButtonUpdateSource {
    Key(KeyDesc),
}

impl ButtonUpdateSource {
    fn try_from(desc: InputDesc) -> Option<ButtonUpdateSource> {
        match desc {
            InputDesc::Key(keydesc) => Some(ButtonUpdateSource::Key(keydesc)),
            InputDesc::Signal(_) => None,
        }
    }
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

/// A description of events that can change the state of a signal-type input.
pub enum SignalUpdateSource {
    Signal(Signal),
    Key(KeyDesc),
}

impl From<KeyDesc> for SignalUpdateSource {
    fn from(keydesc: KeyDesc) -> SignalUpdateSource {
        SignalUpdateSource::Key(keydesc)
    }
}

impl From<Key> for SignalUpdateSource {
    fn from(key: Key) -> SignalUpdateSource {
        SignalUpdateSource::Key(KeyDesc::new(key))
    }
}

impl From<Signal> for SignalUpdateSource {
    fn from(signal: Signal) -> SignalUpdateSource {
        SignalUpdateSource::Signal(signal)
    }
}

impl Into<InputDesc> for SignalUpdateSource {
    fn into(self: SignalUpdateSource) -> InputDesc {
        match self {
            SignalUpdateSource::Key(desc) => InputDesc::Key(desc),
            SignalUpdateSource::Signal(note) => InputDesc::Signal(note),
        }
    }
}


/// Associates abstract input descriptions with game actions, and maps input
/// events to updates to a representation of the game's full action state.
#[derive(Debug, Clone)]
pub struct InputMap<BI: InputIndex, NI: InputIndex> {
    buttons: HashMap<InputDesc, Vec<BI>>,
    signals: HashMap<InputDesc, Vec<NI>>,
}

impl<BI: InputIndex, NI: InputIndex> InputMap<BI, NI> {
    /// Creates a new input map.
    pub fn new() -> InputMap<BI, NI> {
        InputMap {
            buttons: HashMap::new(),
            signals: HashMap::new(),
        }
    }

    /// Adds a mapping from a button input source to a button action.
    pub fn add_button<D: Into<ButtonUpdateSource>>(&mut self, action: BI, desc: D) {
        self.buttons.entry(desc.into().into()).or_insert_with(Vec::new).push(action);
    }

    /// Adds a mapping from a signal input source to a signal action.
    pub fn add_signal<D: Into<SignalUpdateSource>>(&mut self, action: NI, desc: D) {
        self.signals.entry(desc.into().into()).or_insert_with(Vec::new).push(action);
    }

    /// Returns the ids of the buttons bound by this map.
    pub fn bound_buttons(&self) -> HashSet<BI> {
        self.buttons.values().flat_map(|v| v).map(|id| *id).collect()
    }

    /// Returns the sources
    pub fn button_sources(&self, action: BI) -> Vec<ButtonUpdateSource> {
        self.buttons
            .iter()
            .filter(|&(_, ids)| ids.contains(&action))
            .map(|(d, _)| ButtonUpdateSource::try_from(d.clone()).unwrap())
            .collect()
    }

    /// Adds all sources for button-type inputs that are bound by 'other' but
    /// not by this maps.
    /// This means that if this map doesn't map anything to 'shoot', it will get
    /// bindings from both buttons C and D from the other map.
    pub fn add_unbound_buttons_from(&mut self, other: &InputMap<BI, NI>) {
        let own_buttons = self.bound_buttons();
        for button_id in other.bound_buttons() {
            if !own_buttons.contains(&button_id) {
                for description in other.button_sources(button_id) {
                    self.add_button(button_id, description);
                }
            }
        }
    }

    /// Applies the changes described by the given event to the input state.
    pub fn apply<E, S>(&self, event: &E, state: &mut S)
        where E: DescribeInputChanges,
              S: InputState<ButtonId = BI, SignalId = NI>
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
                    InputChange::Signal(_) => unreachable!(),
                }
            }

            // NOTIFICATION MAPPING
            for signal_id in self.signals.get(&input).into_iter().flat_map(|a| a) {
                let mut signal_received = state.get_signal(&signal_id);
                match change {
                    InputChange::Key(_, state) => {
                        match state {
                            Pressed => {
                                *signal_received = true;
                            }
                            Released | Repeated => {}
                        }
                    }
                    InputChange::Signal(_) => {
                        *signal_received = true;
                    }
                }
            }
        });
    }
}
