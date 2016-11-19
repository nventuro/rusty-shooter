use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;

pub struct DefaultView;

impl View for DefaultView {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let events = &mut context.events;
        let renderer = &mut context.renderer;

        if (events.now.key_escape == Some(true)) || (events.now.quit) {
            return ViewAction::Quit
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        ViewAction::None
    }
}
