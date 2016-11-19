use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;

pub struct MainMenuView;

impl MainMenuView {
    pub fn new(_: &mut Phi) -> MainMenuView {
        MainMenuView
    }
}

impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, _: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if phi.events.key_space {
            return ViewAction::ChangeView{ new_view: Box::new(::views::game::ShipView::new(phi)) };
        }

        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        ViewAction::None
    }
}

