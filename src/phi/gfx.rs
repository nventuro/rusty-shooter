use phi::data::Rectangle;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;


#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}

impl Sprite {
    /// Creates a new sprite by wrapping a `Texture`.
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

    /// Creates a new sprite from an image file located at the given path.
    /// Returns `Some` if the file could be read, and `None` otherwise.
    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    /// Returns a new `Sprite` representing a sub-region of the current one.
    /// The provided `rect` is relative to the currently held region.
    /// Returns `Some` if the `rect` is valid, i.e. included in the current
    /// region, and `None` otherwise.
    pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
        let new_src = Rectangle {
            x: rect.x + self.src.x,
            y: rect.y + self.src.y,
            ..rect
        };

        if self.src.contains(new_src) {
            Some(Sprite {
                tex: self.tex.clone(),
                src: new_src
            })
        } else {
            None
        }
    }

    // Returns the dimensions of the region.
    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }

    pub fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        renderer.copy(&mut self.tex.borrow_mut(), Some(self.src.to_sdl()), Some(dest.to_sdl())).unwrap();
    }
}

pub trait RenderSprite {
    fn render_sprite(&mut self, sprite: &Sprite, dest: Rectangle);
}

impl<'window> RenderSprite for Renderer<'window> {
    fn render_sprite(&mut self, sprite: &Sprite, dest: Rectangle) {
       sprite.render(self, dest);
   }
}
