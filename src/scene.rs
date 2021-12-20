use crate::circles::Circle;
use crate::hashgrid::HashGrid;
use image::RgbImage;
use std::vec::Vec;

pub struct Scene {
    pub cpf: u32, // circles per frame
    pub circles: Vec<Circle>,
    rate: f64,
    size: (f64, f64),
    // hash_grid: HashGrid,
}

impl Scene {
    pub fn new(dims: (u32, u32), cpf: u32, rate: f64) -> Self {
        Scene {
            cpf,
            rate,
            circles: Vec::new(),
            size: (dims.0 as f64, dims.1 as f64),
        }
    }

    pub fn populate_fp(
        &mut self,
        circle_count: u32,
        img: &RgbImage,
        fp: &mut Vec<[usize; 2]>,
    ) -> bool {
        let mut ret = true;
        'outer: for _x in 0..circle_count {
            loop {
                if fp.is_empty() {
                    ret = false;
                    break 'outer;
                }

                let tup = Circle::rand(fp, img);

                if !self.boundaries(&tup.0) && !tup.0.colliding_any(&self.circles) {
                    self.circles.push(tup.0);
                    break;
                }
            }
        }
        ret
    }

    fn find_colliding(&mut self) -> Vec<usize> {
        let mut to_move = Vec::<usize>::new();
        for (i, circle) in self.circles.iter().enumerate() {
            if circle.d && (circle.colliding_any(&self.circles) || self.boundaries(circle)) {
                to_move.push(i);
            }
        }
        to_move
    }

    pub fn boundaries(&self, c: &Circle) -> bool {
        c.x + c.r > self.size.0 as f64
            || c.x - c.r < 0.0
            || c.y + c.r > self.size.1 as f64
            || c.y - c.r < 0.0
    }

    pub fn update(&mut self) {
        let to_stop: Vec<usize> = self.find_colliding();
        for i in to_stop {
            self.circles[i].d = false;
        }
        for c in &mut self.circles {
            if c.d {
                c.change_radius(self.rate);
            }
        }
    }
}
