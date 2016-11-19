use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;

pub struct RedView;

impl View for RedView {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let events = &mut context.events;
        let renderer = &mut context.renderer;

        if (events.now.key_escape == Some(true)) || (events.now.quit) {
            return ViewAction::Quit
        } else if events.now.key_space == Some(true) {
            return ViewAction::ChangeView { new_view: Box::new(BlueView) }
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.clear();

        ViewAction::None
    }
}

pub struct BlueView;

impl View for BlueView {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let events = &mut context.events;
        let renderer = &mut context.renderer;

        if (events.now.key_escape == Some(true)) || (events.now.quit) {
            return ViewAction::Quit
        } else if events.now.key_space == Some(true) {
            return ViewAction::ChangeView { new_view: Box::new(RedView) }
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 255));
        renderer.clear();

        ViewAction::None
    }
}
