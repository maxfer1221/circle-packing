use image::RgbImage;
use sdl2::pixels::Color;

#[derive(Clone, Debug)]
pub struct Circle {
    pub x: f64,       // x: x position in window
    pub y: f64,       // y: y position in window
    pub radius: f64,  // r: circle radius
    pub color: Color, // c: draw color
    pub dynamic: bool,
    pub index: usize,
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Circle {
    pub fn _new(x: f64, y: f64, radius: f64, color: Option<Color>, index: usize) -> Self {
        Circle {
            x,
            y,
            radius,
            index,
            dynamic: true,
            color: match color {
                Some(c) => c,
                None => Color::RGB(255, 255, 255),
            },
        }
    }

    pub fn rand(fp: &mut Vec<[usize; 2]>, img: &RgbImage, index: usize) -> (Self, usize) {
        let _index = (rand::random::<f32>() * fp.len() as f32).floor() as usize;
        let ([x, y], radius) = (fp.swap_remove(_index), 0.0);
        let colors = img.get_pixel(x as u32, y as u32).0;
        let color = Color::RGB(colors[0], colors[1], colors[2]);
        (
            Circle {
                x: x as f64,
                y: y as f64,
                radius,
                color,
                index,
                dynamic: true,
            },
            _index,
        )
    }

    fn dist(&self, c: &Self) -> f64 {
        ((self.x - c.x).powf(2.0) + (self.y - c.y).powf(2.0)).sqrt()
    }

    pub fn colliding(&self, c: &Self) -> bool {
        self.dist(c) - (self.radius + c.radius) < 0.0
    }

    pub fn colliding_any(&self, circles: Vec<&Self>) -> bool {
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
        self.radius += rad;
    }
}
