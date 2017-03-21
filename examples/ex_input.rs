#[macro_use]
extern crate dalgi;
use dalgi::input::*;

input! {
    struct Input {
        [button]
        struct ButtonState<ButtonId> {
            jump,
            shoot,
        }
        
        [notification]
        struct NotificationState<NotificationId> {
            quit,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TestKey {
    Up,
    Down,
    Left,
    Right,
    Space,
}

pub enum TestEvent {
    KeyDown(TestKey),
    KeyUp(TestKey),
}

fn map_key(key: TestKey) -> Key {
    use self::TestKey::*;
    match key {
        Up => Key::Up,
        Down => Key::Down,
        Left => Key::Left,
        Right => Key::Right,
        Space => Key::Space,
    }
}

impl DescribeInputChanges for TestEvent {
    fn describe_changes<F: FnMut(InputChange)>(&self, mut handler: F) {
        use self::TestEvent::*;
        match *self {
            KeyDown(key) => {
                handler(InputChange::Key(KeyDesc::new(map_key(key)), ButtonChange::Pressed))
            }
            KeyUp(key) => {
                handler(InputChange::Key(KeyDesc::new(map_key(key)), ButtonChange::Released))
            }
        }
    }
}

fn main() {
    let mut input = Input::new();
    println!("jump: {:?}", input.button.jump);
    input.button.jump.pressed = true;
    println!("jump: {:?}", input.button.jump);

    input.button.shoot.held = true;

    println!("shoot: {:?}", input.button.shoot);
    let mut map = InputMap::new();

    map.add_button(ButtonId::shoot, Key::Space);
    map.add_button(ButtonId::jump, Key::Up);

    input = Input::new(); // reset
    let shoot_event = TestEvent::KeyDown(TestKey::Space);
    let jump_event = TestEvent::KeyDown(TestKey::Up);
    println!("1| {:?}", input);
    map.apply(&shoot_event, &mut input);
    map.apply(&jump_event, &mut input);
    println!("2| {:?}", input);
    input.advance_frame();
    println!("2| {:?}", input);
    let shoot_end = TestEvent::KeyUp(TestKey::Space);
    map.apply(&shoot_end, &mut input);
    println!("3| {:?}", input);
    input.advance_frame();
    println!("4| {:?}", input);
}
