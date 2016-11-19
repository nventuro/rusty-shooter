use phi::data::Rectangle;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;


#[derive(Clone)]
pub struct Sprite {
    // `Texture`s are only loaded once, but multiple `Sprite`s can be created
    // from it because of the `Rc`. We need `Rc` to hold a `RefCell` because the
    // `render` method requires a mutable `Texture`.
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}

impl Sprite {
    pub fn new(texture: Texture) -> Sprite {
        let tex_query = texture.query();

        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rectangle {
                w: tex_query.width as f64,
                h: tex_query.height as f64,
                x: 0.0,
                y: 0.0,
            }
        }
    }

    /// Creates a new `Sprite` from an image file located at the given path.
    /// Returns `Some` if the file could be read, and `None` otherwise.
    pub fn load(renderer: &Renderer, path: &str) -> Sprite {
        Sprite::new(renderer.load_texture(Path::new(path)).unwrap())
    }

    /// Returns a new `Sprite` representing a sub-region of the current one.
    /// Returns `Some` if the `rect` is valid, i.e. included in the current
    /// region, and `None` otherwise.
    pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
        if self.src.contains(rect) {
            Some(Sprite {
                tex: self.tex.clone(),
                src: rect
            })
        } else {
            None
        }
    }

    /// Returns the dimensions of the source region (which may be smaller
    /// than those of the `Texture`!)
    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }

    /// Renders a `Sprite` to the `dest` region. Only the Sprite's sub-region will
    /// be rendered.
    pub fn render(&self, mut renderer: &mut Renderer, dest: Rectangle) {
        renderer.copy(&mut self.tex.borrow_mut(), Some(self.src.to_sdl()), Some(dest.to_sdl())).unwrap();
    }
}

#[derive(Clone)]
pub struct ParallaxSprite {
    pos: f64,
    /// The amount of pixels moved to the left every second
    vel: f64,
    sprite: Sprite,
}

impl ParallaxSprite {
    pub fn new(texture: Texture, vel: f64) -> ParallaxSprite {
        ParallaxSprite {
            pos: 0.0,
            vel: vel,
            sprite: Sprite::new(texture)
        }
    }

    pub fn load(renderer: &Renderer, path: &str, vel: f64) -> ParallaxSprite {
        ParallaxSprite {
            pos: 0.0,
            vel: vel,
            sprite: Sprite::new(renderer.load_texture(Path::new(path)).unwrap())
        }
    }

    /// Renders the `ParallaxSprite` to `dest` (`None` to use the full window). `elapsed` is the
    /// number of seconds that have passed since the last `render` call.
    pub fn render(&mut self, mut renderer: &mut Renderer, dest: Option<Rectangle>, elapsed: f64) {
        // We define a logical position as depending solely on the time and the
        // dimensions of the image, not on the destination's size.
        let (w, h) = self.sprite.size();
        self.pos = (self.pos + self.vel * elapsed) % w;

        let (rect_w, rect_h) = if dest.is_some() {
            (dest.unwrap().w, dest.unwrap().h)
        } else {
            let (win_w, win_h) = renderer.output_size().unwrap();
            (win_w as f64, win_h as f64)
        };

        // We determine the scale ratio of the rectangle to the sprite. Since we're
        // doing parallax, the ratio is solely determined by the height difference.
        let scale = rect_h as f64 / h;

        // We render as many copies of the image as necessary to fill
        // the rectangle.
        let mut physical_left = -self.pos * scale;

        while physical_left < rect_w as f64 {
            self.sprite.render(&mut renderer, Rectangle {
                x: physical_left,
                y: 0.0,
                w: w * scale,
                h: rect_h as f64,
            });

            physical_left += w * scale;
        }
    }
}
