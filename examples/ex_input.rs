#[macro_use]
extern crate dalgi;
use dalgi::input::*;

input_state! {
    Input {
        jump,
        shoot
    }
    id_enum: ActionId
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

impl DescribeEvent for TestEvent {
    fn describe(&self) -> InputChange {
        use self::TestEvent::*;
        match *self {
            KeyDown(key) => InputChange::Key(KeyDesc::new(map_key(key)), ButtonState::Pressed),
            KeyUp(key) => InputChange::Key(KeyDesc::new(map_key(key)), ButtonState::Released),
        }
    }
}

pub fn main() {
    let mut input = Input::new();
    println!("jump: {:?}", input.jump);
    input.jump.pressed = true;
    println!("jump: {:?}", input.jump);

    {
        let mut shoot = input.get_option(ActionId::shoot);
        shoot.held = true;
    }

    println!("shoot: {:?}", input.shoot);
    let mut mapper = InputMapper::new();

    let shoot = KeyDesc::new(Key::Space);
    mapper.insert(shoot, ActionId::shoot);

    let jump = KeyDesc::new(Key::Up);
    mapper.insert(jump, ActionId::jump);

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
