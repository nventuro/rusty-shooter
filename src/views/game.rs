use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use phi::gfx::{Sprite, ParallaxSprite};
use sdl2::pixels::Color;

use sdl2::render::Renderer;

/// Pixels traveled by the player's ship every second, when it is moving.
const PLAYER_SPEED: f64 = 180.0;

struct Ship {
    pos: Rectangle,
    bounds: Rectangle,
    sprites: Vec<Sprite>,
    stance: ShipStance
}

/// The different states our ship might be in. In the image, they're ordered
/// from left to right, then from top to bottom.
#[derive(Clone, Copy)]
enum ShipStance {
    UpNorm   = 0,
    UpFast   = 1,
    UpSlow   = 2,
    MidNorm  = 3,
    MidFast  = 4,
    MidSlow  = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8
}

impl Ship {
    pub fn new(renderer: &mut Renderer, path: &str, bounds: Rectangle) -> Ship {
        // The spritesheet contains a 3x3 grid with all the stances
        let spritesheet = Sprite::load(renderer, path);
        let (w, h) = spritesheet.size();
        let w = w / 3.0;
        let h = h / 3.0;

        let mut sprites = Vec::with_capacity(9);
        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    w: w,
                    h: h,
                    x: w * x as f64,
                    y: h * y as f64,
                }).unwrap());
            }
        }

        Ship {
            pos: Rectangle {
                w: w,
                h: h,
                x: 64.0,
                y: 64.0
            },
            bounds: bounds,
            sprites: sprites,
            stance: ShipStance::MidNorm
        }
    }

    pub fn update(&mut self, key_up: bool, key_down: bool, key_left: bool, key_right: bool, elapsed: f64) {
        let diagonal =
            (key_up ^ key_down) &&
            (key_left ^ key_right);

        let moved =
            if diagonal { 1.0 / 2.0f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (key_left, key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (key_up, key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let new_pos = Rectangle {
            x: self.pos.x + dx,
            y: self.pos.y + dy,
            .. self.pos
        };

        self.pos = new_pos.restrict_to_bounds(self.bounds);
        self.update_stance(dx, dy);
    }

    fn update_stance(&mut self, dx: f64, dy: f64) {
        self.stance =
            if dx == 0.0 && dy < 0.0       { ShipStance::UpNorm }
            else if dx > 0.0 && dy < 0.0   { ShipStance::UpFast }
            else if dx < 0.0 && dy < 0.0   { ShipStance::UpSlow }
            else if dx == 0.0 && dy == 0.0 { ShipStance::MidNorm }
            else if dx > 0.0 && dy == 0.0  { ShipStance::MidFast }
            else if dx < 0.0 && dy == 0.0  { ShipStance::MidSlow }
            else if dx == 0.0 && dy > 0.0  { ShipStance::DownNorm }
            else if dx > 0.0 && dy > 0.0   { ShipStance::DownFast }
            else if dx < 0.0 && dy > 0.0   { ShipStance::DownSlow }
            else { unreachable!() };
    }

    pub fn render(&self, mut renderer: &mut Renderer) {
        self.sprites[self.stance as usize].render(&mut renderer, self.pos);
    }
}

pub struct ShipView {
    player: Ship,

    bg_back: ParallaxSprite,
    bg_middle: ParallaxSprite,
    foreground: ParallaxSprite,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        let player_bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1
        };

        ShipView {
            player: Ship::new(&mut phi.renderer, "assets/spaceship.png", player_bounds),
            bg_back: ParallaxSprite::load(&mut phi.renderer, "assets/starBG.png", 20.0),
            bg_middle: ParallaxSprite::load(&mut phi.renderer, "assets/starMG.png", 40.0),
            foreground: ParallaxSprite::load(&mut phi.renderer, "assets/starFG.png", 80.0),
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        self.player.update(phi.events.key_up, phi.events.key_down, phi.events.key_left, phi.events.key_right, elapsed);

        // Clear the scene
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render the Backgrounds
        self.bg_back.render(&mut phi.renderer, None, elapsed);
        self.bg_middle.render(&mut phi.renderer, None, elapsed);

        // Render the ship
        self.player.render(&mut phi.renderer);

        // Render the foreground
        self.foreground.render(&mut phi.renderer, None, elapsed);

        ViewAction::None
    }
}
