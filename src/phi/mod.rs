#[macro_use]
mod events;

use sdl2::render::Renderer;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    other: {
        quit: Quit { .. }
    }
}

pub enum ViewAction {
    None,
    ChangeView { new_view: Box<View> },
    Quit,
}

pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

pub trait View {
    /// Called on every frame to take care of both the logic and
    /// the rendering of the current view.
    ///
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

pub fn spawn<F>(title: &str, init: F)
where F: Fn(&mut Phi) -> Box<View> {
    // Initialize SDL2
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();

    // Create the window
    let window = video.window(title, 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    // Create the context
    let mut context = Phi {
        events: Events::new(sdl_context.event_pump().unwrap()),
        renderer: window.renderer()
            .accelerated()
            .build().unwrap(),
    };

    // Create the initial view
    let mut current_view = init(&mut context);

    // Frame timing
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0;

    loop {
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        // Lock to the intended framerate
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }


        // Logic & rendering

        context.events.pump();

        match current_view.render(&mut context, elapsed) {
            ViewAction::None =>
                context.renderer.present(),

            ViewAction::ChangeView { new_view } =>
                current_view = new_view,

            ViewAction::Quit =>
                break
        }
    }
}
