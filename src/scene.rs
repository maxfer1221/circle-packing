use image::RgbImage;
use rand::{Rng, rngs::ThreadRng};
use sdl2::pixels::Color;
use std::vec::Vec;
use crate::circles::Circle; 
 
pub struct Scene {
    pub cpf: u32, // circles per frame
    pub circles: Vec<Circle>,
    pub dynamic: Vec<Circle>,
    rate: f64,
    size: (f64, f64),
    //hash_grid: HashGrid,
}

impl Scene {
    pub fn new(dims: (u32, u32), cpf: u32, rate: f64) -> Self {
        Scene {
            cpf: cpf,
            circles: Vec::new(),
            dynamic: Vec::new(),
            rate: rate,
            size: (dims.0 as f64, dims.1 as f64),
        }
    }

    pub fn populate(&mut self, i: u32, rng: &mut ThreadRng, img: &RgbImage) -> bool {
        for _x in 0..i {
            let mut attempts = 0;
            let (x, y, r): (f64, f64, f64) = (rng.gen_range(0.0..self.size.0), rng.gen_range(0.0..self.size.1), 1.0);
            let mut colors = img.get_pixel(x as u32, y as u32).0;
            let mut c = Circle::new(x, y, r, Some(Color::RGB(colors[0], colors[1], colors[2])));
            while self.boundaries(&c) || c.colliding_any(&self.circles) || c.colliding_any(&self.dynamic) && attempts < 1000 {
                attempts += 1;
                let (x, y, r) = (rng.gen_range(0.0..self.size.0), rng.gen_range(0.0..self.size.1), 1.0);
                colors = img.get_pixel(x as u32, y as u32).0;
                c = Circle::new(x, y, r, Some(Color::RGB(colors[0], colors[1], colors[2])));
            }
            if attempts >= 1000 {
                return false
            }
            self.dynamic.push(c);
        }
        true
    }
    
    pub fn populate_fp(&mut self, i: u32, rng: &mut ThreadRng, img: &RgbImage, fp: &mut Vec<[usize; 2]>) -> bool {
        if fp.len() == 0 {
            return false;
        }
        for _x in 0..i {
            let mut attempts = 0;
            let mut index = (rand::random::<f32>() * fp.len() as f32).floor() as usize;
            let mut el = fp[index];
            let (x, y, r): (f64, f64, f64) = (el[0] as f64, el[1] as f64, 0.0);
            let mut colors = img.get_pixel(x as u32, y as u32).0;
            let mut c = Circle::new(x, y, r, Some(Color::RGB(colors[0], colors[1], colors[2])));
            while self.boundaries(&c) || c.colliding_any(&self.circles) || c.colliding_any(&self.dynamic) && attempts < 1000 {
                attempts += 1;
                index = (rand::random::<f32>() * fp.len() as f32).floor() as usize;
                el = fp[index];
                let (x, y, r): (f64, f64, f64) = (el[0] as f64, el[1] as f64, 1.0);
                colors = img.get_pixel(x as u32, y as u32).0;
                c = Circle::new(x, y, r, Some(Color::RGB(colors[0], colors[1], colors[2])));
            }
            if attempts >= 1000 {
                return false
            }
            self.dynamic.push(c);
            fp.swap_remove(index);
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