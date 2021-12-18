#[derive(Clone)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle {
            x: x,
            y: y,
            r: r,
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
            if self.colliding(c) {
                return true;
            }
        }
        false
    }

    pub fn change_radius(&mut self, rad: f64) {
        self.r += rad;
    }

    pub fn copy(&self) -> Self {
        Circle {
            x: self.x,
            y: self.y,
            r: self.r,
        }
    }
}