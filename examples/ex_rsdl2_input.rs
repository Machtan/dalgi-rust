//! Requires the 'rsdl2-support' feature!

#[macro_use]
extern crate dalgi;
#[cfg(feature = "rsdl2-support")]
extern crate rsdl2;

#[cfg(feature = "rsdl2-support")]
use rsdl2::{Rect};
use dalgi::input::*;
use std::time::Duration;
use std::thread;

input_state! {
    Input {
        jump,
        shoot,
        left,
        right
    }
    id_enum: ActionId
}

#[cfg(feature = "rsdl2-support")]
fn main() {
    // Setup SDL2
    let context = rsdl2::init().everything().finish().expect("init failed");
    let mut event_context = context.events().expect("Event subsystem not initialized");
    let video_context = context.video().expect("Video subsystem not initialized");
    let window = video_context.build_window()
        .title("SDL Game")
        .center(true, true)
        .finish()
        .expect("Could not create window");
    let renderer = window.build_renderer().finish().expect("Could not build renderer");
    let clear_color = (255, 200, 220);
    let cornflower = (154, 206, 235);
    let mut rect = Rect::new(100, 100, 100, 100);
        
    // Setup input
    let mut mapper = InputMapper::new();
    mapper.add(ActionId::shoot, Key::Space);
    mapper.add(ActionId::jump, Key::Up);
    mapper.add(ActionId::right, Key::Right);
    mapper.add(ActionId::left, Key::Left);

    let mut input = Input::new(); // reset

    'main: loop {
        use rsdl2::events::EventKind::*;
        
        input.advance_frame();
        
        for event in event_context.events() {
            match event.kind {
                Quit => {
                    println!("User-requested Quit!");
                    break 'main;
                }
                other => {
                    mapper.map(&other, &mut input);
                }
            }
        }
        
        if input.jump.pressed {
            println!("Jump!");
            rect.move_by(0, -5);
        }
        if input.jump.released && ! input.jump.held {
            rect.move_by(0, 5);
        }
        if input.shoot.pressed {
            println!("Shoot!");
        }
        match (input.left.held, input.right.held) {
            (true, true) => {}, //println!("Holding still!"),
            (true, false) => {
                rect.move_by(-10, 0);
            },
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


