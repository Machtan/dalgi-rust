extern crate dalgi;
use dalgi::input::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ButtonId {
    Jump,
    Shoot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SignalId {
    Quit,
}

#[derive(Debug, Clone, Default)]
struct ButtonState {
    pub jump: ButtonValue,
    pub shoot: ButtonValue,
}

#[derive(Debug, Clone, Default)]
struct SignalState {
    pub quit: bool,
}

#[derive(Debug, Clone, Default)]
struct Input {
    pub button: ButtonState,
    pub signal: SignalState,
}

impl AdvanceFrame for Input {
    fn advance_frame(&mut self) {
        self.button.jump.advance_frame();
        self.button.shoot.advance_frame();
        self.signal.quit.advance_frame();
    }
}

impl InputState for Input {
    type ButtonId = ButtonId;
    type SignalId = SignalId;

    fn get_button<'a>(&'a mut self, id: &Self::ButtonId) -> &'a mut ButtonValue {
        match id {
            &ButtonId::Jump => &mut self.button.jump,
            &ButtonId::Shoot => &mut self.button.shoot,
        }
    }

    fn get_signal<'a>(&'a mut self, id: &Self::SignalId) -> &'a mut bool {
        match id {
            &SignalId::Quit => &mut self.signal.quit,
        }
    }
}

fn main() {
    let mut input = Input::default();
    let mut map = InputMap::new();
    map.add_button(ButtonId::Jump, Key::Up);
    map.add_button(ButtonId::Shoot, Key::Space);
    map.add_signal(SignalId::Quit, Signal::QuitRequest);

    // TODO: Read from stdin and use that to simulate events.

    println!("1 Jump: {:?}", input.button.jump);
    let change = InputChange::Key(KeyDesc::new(Key::Up), ButtonChange::Pressed);
    map.apply(&change, &mut input);
    println!("1 Jump: {:?}", input.button.jump);
    input.advance_frame();
    println!("2 Jump: {:?}", input.button.jump);
    let change = InputChange::Key(KeyDesc::new(Key::Up), ButtonChange::Released);
    map.apply(&change, &mut input);
    println!("2 Jump: {:?}", input.button.jump);

    let change = InputChange::Key(KeyDesc::new(Key::Space), ButtonChange::Pressed);
    map.apply(&change, &mut input);
    println!("2 Shoot: {:?}", input.button.shoot);

    println!("2 Quit: {}", input.signal.quit);
    let change = InputChange::Signal(Signal::QuitRequest);
    map.apply(&change, &mut input);
    println!("2 Quit: {}", input.signal.quit);
}
