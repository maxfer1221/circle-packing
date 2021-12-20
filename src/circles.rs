use sdl2::pixels::Color;

#[derive(Clone, Debug)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
    pub c: Color,
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64, c: Option<Color>) -> Self {
        Circle {
            x: x,
            y: y,
            r: r,
            c: match c {
                Some(c) => c,
                None => {
                    Color::RGB(255, 255, 255)
                }
            },
        }
    }

    fn dist(&self, c: &Self) -> f64 {
        ((self.x - c.x).powf(2.0) + (self.y - c.y).powf(2.0)).sqrt()
    }

    pub fn colliding(&self, c: &Self) -> bool {
        self.dist(c) - (self.r + c.r) < 0.0
    }

    pub fn colliding_any(&self, circles: &Vec<Self>) -> bool {
        for c in circles {
            if self != c && self.colliding(c) {
                return true;
            }
        }
        false
    }

    pub fn change_radius(&mut self, rad: f64) {
        self.r += rad;
    }
}