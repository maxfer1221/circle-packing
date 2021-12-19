use crate::circles::Circle;
use rand::{Rng, rngs::ThreadRng};
use std::vec::Vec;

pub struct Scene {
    pub circles: Vec<Circle>,
    pub dynamic: Vec<Circle>,
    rate: f64,
    size: (f64, f64),
    //hash_grid: HashGrid,
}

impl Scene {
    pub fn new(dims: (u32, u32), rate: f64) -> Self {
        Scene {
            circles: Vec::new(),
            dynamic: Vec::new(),
            rate: rate,
            size: (dims.0 as f64, dims.1 as f64),
        }
    }

    pub fn populate(&mut self, i: u32, rng: &mut ThreadRng) -> bool {
        for _x in 0..i {
            let mut attempts = 0;
            let mut c = Circle::new(rng.gen_range(0.0..self.size.0), rng.gen_range(0.0..self.size.1), rng.gen_range(1.0..5.0), None);
            while self.boundaries(&c) || c.colliding_any(&self.circles) || c.colliding_any(&self.dynamic) && attempts < 1000 {
                attempts += 1;
                c = Circle::new(rng.gen_range(20.0..780.0), rng.gen_range(20.0..780.0), rng.gen_range(5.0..20.0), None);
            }
            if attempts == 1000 {
                return false
            }
            self.dynamic.push(c);
        }
        true
    }

    fn find_colliding(&mut self) -> Vec<usize> {
        let mut to_move = Vec::<usize>::new();
        for (i, circle) in self.dynamic.iter().enumerate() {
            if circle.colliding_any(&self.circles) {
                to_move.push(i);
            } else if circle.colliding_any(&self.dynamic) {
                to_move.push(i);
            } else if self.boundaries(&circle) {
                to_move.push(i);
            }
        }
        to_move
    }

    pub fn boundaries(&self, c: &Circle) -> bool {
        c.x + c.r > self.size.0 as f64 || c.x - c.r < 0.0 || c.y + c.r > self.size.1 as f64 || c.y - c.r < 0.0
    }

    pub fn update(&mut self) {
        let mut to_move: Vec<usize> = self.find_colliding();
        to_move.reverse();
        for i in to_move {
            let c = self.dynamic.swap_remove(i);
            self.circles.push(c);
        }
        for dynamic in &mut self.dynamic {
            dynamic.change_radius(self.rate);
        }
    }
}