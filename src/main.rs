extern crate sdl2;

#[macro_use]
mod events;

struct_events! {
    keyboard: {
        key_escape: Escape
    },
    other: {
        quit: Quit { .. }
    }
}

use sdl2::pixels::Color;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("rusty-shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if (events.now.key_escape == Some(true)) || (events.now.quit) {
            break;
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
