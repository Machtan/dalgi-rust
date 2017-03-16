use rsdl2::events::{Event, EventKind};
use rsdl2::{Keycode, Scancode, Keysym};
use input::*;

fn map_keycode(key: Keycode) -> Key {
    use rsdl2::Keycode::*;
    match key {
        Up => Key::Up,
        Down => Key::Down,
        Left => Key::Left,
        Right => Key::Right,
        Space => Key::Space,
        Return => Key::Return,
        One => Key::One,
        Two => Key::Two,
        Three => Key::Three,
        Four => Key::Four,
        _ => Key::Other(key as i32),
    }
}

fn map_scancode(key: Scancode) -> Key {
    use rsdl2::Scancode::*;
    match key {
        Up => Key::Up,
        Down => Key::Down,
        Left => Key::Left,
        Right => Key::Right,
        Space => Key::Space,
        Return => Key::Return,
        One => Key::One,
        Two => Key::Two,
        Three => Key::Three,
        Four => Key::Four,
        _ => Key::Other(key as i32),
    }
}

fn key_change(keysym: Keysym, state: ButtonChange) -> InputChange {
    InputChange::Key(KeyDesc::new(map_keycode(keysym.keycode)),state)
}

fn scan_change(keysym: Keysym, state: ButtonChange) -> InputChange {
    InputChange::Key(KeyDesc::new(map_scancode(keysym.scancode)).scancode(), state)
}

impl DescribeInputChanges for EventKind {
    fn describe_changes<F: FnMut(InputChange)>(&self, mut handler: F) {
        use rsdl2::events::EventKind::*;
        match *self {
            KeyDown(keysym) => {
                handler(key_change(keysym, ButtonChange::Pressed));
                handler(scan_change(keysym, ButtonChange::Pressed));
            },
            KeyRepeat(keysym) => {
                handler(key_change(keysym, ButtonChange::Repeated));
                handler(scan_change(keysym, ButtonChange::Repeated));
            },
            KeyUp(keysym) => {
                handler(key_change(keysym, ButtonChange::Released));
                handler(scan_change(keysym, ButtonChange::Released));
            },
            Quit => {
                handler(Notification::QuitRequest.into());
            }
            _ => {},
        }
    }
}

impl DescribeInputChanges for Event {
    fn describe_changes<F: FnMut(InputChange)>(&self, handler: F) {
        self.kind.describe_changes(handler);
    }
}
