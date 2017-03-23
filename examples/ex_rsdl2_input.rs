//! Requires the 'rsdl2-support' feature!

#[macro_use]
extern crate dalgi;
#[cfg(feature = "rsdl2-support")]
extern crate rsdl2;

#[cfg(feature = "rsdl2-support")]
use rsdl2::Rect;
use dalgi::input::*;
use std::time::Duration;
use std::thread;

input! {
    pub struct Input {
        [button]
        pub struct ButtonState<ButtonId> {
            jump,
            shoot,
            left,
            right,
        }
        
        [signal]
        pub struct SignalState<SignalId> {
            quit,
        }
    }
}

#[cfg(feature = "rsdl2-support")]
fn main() {
    // Setup SDL2
    let context = rsdl2::init().everything().finish().expect("init failed");
    let mut event_context = context.events().expect("Event subsystem not initialized");
    let video_context = context.video().expect("Video subsystem not initialized");
    let mut window = video_context.build_window()
        .title("SDL Game")
        .center(true, true)
        .finish()
        .expect("Could not create window");
    let mut renderer = window.build_renderer().finish().expect("Could not build renderer");
    let clear_color = (255, 200, 220);
    let cornflower = (154, 206, 235);
    let mut rect = Rect::new(100, 100, 100, 100);

    // Setup input
    let mut map = InputMap::new();
    map.add_button(ButtonId::shoot, Key::Space);
    map.add_button(ButtonId::jump, Key::Up);
    map.add_button(ButtonId::right, Key::Right);
    map.add_button(ButtonId::left, Key::Left);
    map.add_signal(SignalId::quit, Signal::QuitRequest);

    let mut input = Input::new();

    'main: loop {
        input.advance_frame();

        for event in event_context.events() {
            map.apply(&event, &mut input);
        }

        if input.signal.quit {
            break 'main;
        }

        if input.button.jump.pressed {
            println!("Jump!");
            rect.move_by(0, -5);
        }
        if input.button.jump.released && !input.button.jump.held {
            rect.move_by(0, 5);
        }
        if input.button.shoot.pressed {
            println!("Shoot!");
        }
        match (input.button.left.held, input.button.right.held) {
            (true, true) => {} //println!("Holding still!"),
            (true, false) => {
                rect.move_by(-10, 0);
            }
            (false, true) => {
                rect.move_by(10, 0);
            }
            _ => {}
        }

        renderer.color(clear_color).clear().unwrap();
        renderer.color(cornflower).fill_rect(rect).unwrap();

        renderer.present();

        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(not(feature = "rsdl2-support"))]
fn main() {
    println!("Run the example with '--features rsdl2-support'!");
    std::process::exit(1);
}
