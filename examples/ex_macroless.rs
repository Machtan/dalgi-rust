extern crate dalgi;
use dalgi::input::*;


#[derive(Debug, Clone, Copy)]
enum ButtonId {
    Jump,
    Shoot,
}

#[derive(Debug, Clone, Copy)]
enum NotificationId {
    Quit,
}

#[derive(Debug, Clone, Default)]
struct ButtonState {
    pub jump: ButtonValue,
    pub shoot: ButtonValue,
}

#[derive(Debug, Clone, Default)]
struct NotificationState {
    pub quit: bool,
}

#[derive(Debug, Clone, Default)]
struct Input {
    pub button: ButtonState,
    pub notification: NotificationState,
}

impl AdvanceFrame for Input {
    fn advance_frame(&mut self) {
        self.button.jump.advance_frame();
        self.button.shoot.advance_frame();
        self.notification.quit.advance_frame();
    }
}

impl InputState for Input {
    type ButtonId = ButtonId;
    type NotificationId = NotificationId;

    fn get_button<'a>(&'a mut self, id: &Self::ButtonId) -> &'a mut ButtonValue {
        match id {
            &ButtonId::Jump => &mut self.button.jump,
            &ButtonId::Shoot => &mut self.button.shoot,
        }
    }

    fn get_notification<'a>(&'a mut self, id: &Self::NotificationId) -> &'a mut bool {
        match id {
            &NotificationId::Quit => &mut self.notification.quit,
        }
    }
}

fn main() {
    let mut input = Input::default();
    let mut mapper = InputMapper::new();
    mapper.add_button(ButtonId::Jump, Key::Up);
    mapper.add_button(ButtonId::Shoot, Key::Space);
    mapper.add_notification(NotificationId::Quit, Notification::QuitRequest);

    // TODO: Read from stdin and use that to simulate events.

    println!("1 Jump: {:?}", input.button.jump);
    let change = InputChange::Key(KeyDesc::new(Key::Up), ButtonChange::Pressed);
    mapper.map(&change, &mut input);
    println!("1 Jump: {:?}", input.button.jump);
    input.advance_frame();
    println!("2 Jump: {:?}", input.button.jump);
    let change = InputChange::Key(KeyDesc::new(Key::Up), ButtonChange::Released);
    mapper.map(&change, &mut input);
    println!("2 Jump: {:?}", input.button.jump);

    let change = InputChange::Key(KeyDesc::new(Key::Space), ButtonChange::Pressed);
    mapper.map(&change, &mut input);
    println!("2 Shoot: {:?}", input.button.shoot);

    println!("2 Quit: {}", input.notification.quit);
    let change = InputChange::Notification(Notification::QuitRequest);
    mapper.map(&change, &mut input);
    println!("2 Quit: {}", input.notification.quit);
}
