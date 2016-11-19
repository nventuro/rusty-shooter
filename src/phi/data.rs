use sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates an SDL-compatible Rect equivalent to `self`.
    /// Panics if it could not be created, for example if a
    /// coordinate of a corner overflows an `i32`.
    pub fn to_sdl(self) -> Option<SdlRect> {
        // Reject negative width and height
        assert!(self.w >= 0.0 && self.h >= 0.0);

        // SdlRect::new : `(i32, i32, u32, u32) -> Result<Option<SdlRect>>`
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
            .unwrap()
    }

    /// Returns a (perhaps moved) rectangle which is contained by a `parent`
    /// rectangle. If it can indeed be moved to fit, return `Some(result)`;
    /// otherwise, return `None`.
    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        // It must be smaller than the parent rectangle to fit in it.
        if self.w > parent.w || self.h > parent.h {
            return None;
        }

        Some(Rectangle {
            w: self.w,
            h: self.h,
            x: if self.x < parent.x { parent.x }
               else if self.x + self.w >= parent.x + parent.w { parent.x + parent.w - self.w }
               else { self.x },
           y: if self.y < parent.y { parent.y }
               else if self.y + self.h >= parent.y + parent.h { parent.y + parent.h - self.h }
               else { self.y }
        })
    }

    pub fn contains(&self, other: Rectangle) -> bool {
        let x_min = other.x;
        let x_max = other.x + other.w;
        let y_min = other.y;
        let y_max = other.y + other.h;

        x_min >= self.x && x_min <= self.x + self.w &&
        x_max >= self.x && x_max <= self.x + self.w &&
        y_min >= self.y && y_min <= self.y + self.h &&
        y_max >= self.y && y_max <= self.y + self.h
    }

    pub fn overlaps(&self, other: Rectangle) -> bool {
        self.x < other.x + other.w &&
        other.x < self.x + self.w &&
        self.y < other.y + other.h &&
        other.y < self.y + self.h
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn rect_basics() {
        use phi::data::Rectangle;

        let bounding_rect = Rectangle {
            x: 1.0,
            y: 1.0,
            w: 1.0,
            h: 1.0
        };

        // Outside rectangles
        let outside_rect = Rectangle { x: 0.0, .. bounding_rect };
        assert!(!bounding_rect.contains(outside_rect));
        assert!(!bounding_rect.overlaps(outside_rect));

        let outside_rect = Rectangle { y: 0.0, .. bounding_rect };
        assert!(!bounding_rect.contains(outside_rect));
        assert!(!bounding_rect.overlaps(outside_rect));

        let outside_rect = Rectangle { x: 2.0, .. bounding_rect };
        assert!(!bounding_rect.contains(outside_rect));
        assert!(!bounding_rect.overlaps(outside_rect));

        let outside_rect = Rectangle { y: 2.0, .. bounding_rect };
        assert!(!bounding_rect.contains(outside_rect));
        assert!(!bounding_rect.overlaps(outside_rect));

        // Overlapping rectangles
        let overlap_rect = Rectangle { x: 0.5, y: 0.5, .. bounding_rect };
        assert!(!bounding_rect.contains(overlap_rect));
        assert!(bounding_rect.overlaps(overlap_rect));

        let overlap_rect = Rectangle { x: 1.5, y: 0.5, .. bounding_rect };
        assert!(!bounding_rect.contains(overlap_rect));
        assert!(bounding_rect.overlaps(overlap_rect));

        let overlap_rect = Rectangle { x: 0.5, y: 1.5, .. bounding_rect };
        assert!(!bounding_rect.contains(overlap_rect));
        assert!(bounding_rect.overlaps(overlap_rect));

        let overlap_rect = Rectangle { x: 1.5, y: 1.5, .. bounding_rect };
        assert!(!bounding_rect.contains(overlap_rect));
        assert!(bounding_rect.overlaps(overlap_rect));

        // Inside rectangles

        let inside_rect = Rectangle { x: 1.5, y: 1.5, w: 0.5, h: 0.5 };
        assert!(bounding_rect.contains(inside_rect));
        assert!(bounding_rect.overlaps(inside_rect));
    }
}
