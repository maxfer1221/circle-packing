use image::RgbImage;
use sdl2::pixels::Color;

#[derive(Clone, Debug)]
pub struct Circle {
    pub x: f64,   // x: x position in window
    pub y: f64,   // y: y position in window
    pub r: f64,   // r: circle radius
    pub c: Color, // c: draw color
    pub d: bool,  // d: whether it should expand
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Circle {
    pub fn _new(x: f64, y: f64, r: f64, c: Option<Color>) -> Self {
        Circle {
            x,
            y,
            r,
            c: match c {
                Some(c) => c,
                None => Color::RGB(255, 255, 255),
            },
            d: true,
        }
    }

    pub fn rand(fp: &mut Vec<[usize; 2]>, img: &RgbImage) -> (Self, usize) {
        let index = (rand::random::<f32>() * fp.len() as f32).floor() as usize;
        let ([x, y], r) = (fp.swap_remove(index), 0.0);
        let colors = img.get_pixel(x as u32, y as u32).0;
        let c = Color::RGB(colors[0], colors[1], colors[2]);
        (
            Circle {
                x: x as f64,
                y: y as f64,
                r,
                c,
                d: true,
            },
            index,
        )
    }

    fn dist(&self, c: &Self) -> f64 {
        ((self.x - c.x).powf(2.0) + (self.y - c.y).powf(2.0)).sqrt()
    }

    pub fn colliding(&self, c: &Self) -> bool {
        self.dist(c) - (self.r + c.r) < 0.0
    }

    pub fn colliding_any(&self, circles: &[Self]) -> bool {
        let mut ret = false;
        for c in circles {
            if self != c && self.colliding(c) {
                ret = true;
                break;
            }
        }
        ret
    }

    pub fn change_radius(&mut self, rad: f64) {
        self.r += rad;
    }
}
