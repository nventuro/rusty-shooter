use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::{Sprite, RenderSprite};
use sdl2::pixels::Color;

struct Action {
    /// The function which should be executed if the action is chosen.
    func: Box<Fn(&mut Phi) -> ViewAction>,

    /// The sprite which is rendered when the player does not focus on this
    /// action's label.
    idle_sprite: Sprite,

    /// The sprite which is rendered when the player "focuses" a label with the
    /// directional keys.
    hover_sprite: Sprite,
}

impl Action {
    fn new(phi: &mut Phi, label: &'static str, func: Box<Fn(&mut Phi) -> ViewAction>) -> Action {
        Action {
            func: func,
            idle_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf", 32, Color::RGB(220, 220, 220)).unwrap(),
            hover_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf", 38, Color::RGB(255, 255, 255)).unwrap(),
        }
    }
}

pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,
}

impl MainMenuView {
    pub fn new(phi: &mut Phi) -> MainMenuView {
        MainMenuView {
            actions: vec![
                Action::new(phi, "New Game", Box::new(|phi| {
                    ViewAction::ChangeView { new_view: Box::new(::views::game::ShipView::new(phi)) }
                })),
                Action::new(phi, "Quit", Box::new(|_| {
                    ViewAction::Quit
                })),
            ],

            selected: 0,
        }
    }
}

impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, _: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if phi.events.now.key_space == Some(true) {
            return (self.actions[self.selected as usize].func)(phi);
        }

        if phi.events.now.key_down == Some(true) {
            self.selected += 1;
            if self.selected >= self.actions.len() as i8 {
                self.selected = 0;
            }
        } else if phi.events.now.key_up == Some(true) {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = (self.actions.len() - 1) as i8;
            }
        }

        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        for (i, action) in self.actions.iter().enumerate() {
            let sprite_to_render = if self.selected == i as i8 {
                &action.hover_sprite
            } else {
                &action.idle_sprite
            };

            let (w, h) = sprite_to_render.size();

            let (win_w, win_h) = phi.output_size();
            phi.renderer.render_sprite(sprite_to_render, Rectangle {
                x: (win_w - w) / 2.0,
                //? We place every element under the previous one.
                y: (win_h - h) / 2.0 + h * 1.5 * i as f64,
                w: w,
                h: h,
            });
        }

        ViewAction::None
    }
}

