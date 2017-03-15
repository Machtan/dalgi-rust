#[macro_use]
extern crate dalgi;
use dalgi::input::*;

input_state! {
    Input {
        buttons: {
            jump,
            shoot,
        }
        notifications: {}
    }
    id_enum: ActionId;
    mod_name: zzz;
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

impl DescribeInputChange for TestEvent {
    fn describe_changes<F: FnMut(InputChange)>(&self, mut handler: F) {
        use self::TestEvent::*;
        match *self {
            KeyDown(key) => handler(InputChange::Key(KeyDesc::new(map_key(key)), ButtonChange::Pressed)),
            KeyUp(key) => handler(InputChange::Key(KeyDesc::new(map_key(key)), ButtonChange::Released)),
        }
    }
}

pub fn main() {
    let mut input = Input::new();
    println!("jump: {:?}", input.jump);
    input.jump.pressed = true;
    println!("jump: {:?}", input.jump);

    if let InputRef::Button(state) = input.get_option(ActionId::shoot) {
        state.held = true;
    }

    println!("shoot: {:?}", input.shoot);
    let mut mapper = InputMapper::new();

    mapper.add(ActionId::shoot, Key::Space);
    mapper.add(ActionId::jump, Key::Up);

    let mut input = Input::new(); // reset
    let shoot_event = TestEvent::KeyDown(TestKey::Space);
    let jump_event = TestEvent::KeyDown(TestKey::Up);
    println!("Input #0: {:?}", input);
    mapper.map(&shoot_event, &mut input);
    mapper.map(&jump_event, &mut input);
    println!("Input #1: {:?}", input);
    input.advance_frame();
    println!("Input #2: {:?}", input);
    let shoot_end = TestEvent::KeyUp(TestKey::Space);
    mapper.map(&shoot_end, &mut input);
    println!("Input #2: {:?}", input);
    input.advance_frame();
    println!("Input #3: {:?}", input);
}
