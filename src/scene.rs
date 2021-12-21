use crate::circles::Circle;
use crate::hashgrid::{Dynamic, HashGrid, HashGridHelper, Indexed};
use image::RgbImage;
use std::ops::Range;
use std::vec::Vec;

impl HashGridHelper for Circle {
    fn get_range(&self, c_d: (f64, f64)) -> (Range<usize>, Range<usize>) {
        let (x_min, x_max) = (
            ((self.x - self.radius) / c_d.0).floor(),
            ((self.x + self.radius) / c_d.0).ceil(),
        );
        let (y_min, y_max) = (
            ((self.y - self.radius) / c_d.1).floor(),
            ((self.y + self.radius) / c_d.1).ceil(),
        );

        // println!(
        //     "{:?}",
        //     (
        //         x_min as usize..x_max as usize,
        //         y_min as usize..y_max as usize,
        //     )
        // );
        (
            x_min as usize..x_max as usize,
            y_min as usize..y_max as usize,
        )
    }
}

impl Dynamic for Circle {
    fn is_dynamic(&self) -> bool {
        self.dynamic
    }
}

impl Indexed for Circle {
    fn index(&self) -> usize {
        self.index
    }
}

pub struct Scene {
    pub cpf: u32, // circles per frame
    pub hashgrid: HashGrid<Circle>,
    pub rate: f64,
    pub size: (f64, f64),
}

impl Scene {
    pub fn new(dims: (u32, u32), cpf: u32, rate: f64, cell_dims: (f64, f64)) -> Self {
        Scene {
            cpf,
            rate,
            hashgrid: HashGrid::new(dims, cell_dims),
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
                let tup = Circle::rand(fp, img, self.hashgrid.all_elements.len());

                // println!();
                if !self.boundaries(&tup.0)
                    && !tup.0.colliding_any(
                        self.hashgrid
                            .elements_from_range(tup.0.get_range(self.hashgrid.cell_dimensions)),
                    )
                {
                    self.hashgrid.all_elements.push(tup.0);
                    break;
                }
            }
        }
        ret
    }

    fn find_colliding(&mut self) -> Vec<usize> {
        let mut to_move = Vec::<usize>::new();
        for (i, circle) in self.hashgrid.all_elements.iter().enumerate() {
            // println!(
            //     "{:?}",
            //     self.hashgrid
            //         .elements_from_range(circle.get_range(self.hashgrid.cell_dimensions))
            // );
            if circle.dynamic
                && (circle.colliding_any(
                    self.hashgrid
                        .elements_from_range(circle.get_range(self.hashgrid.cell_dimensions)),
                ) || self.boundaries(circle))
            {
                to_move.push(i);
            }
        }
        to_move
    }

    pub fn boundaries(&self, c: &Circle) -> bool {
        c.x + c.radius > self.size.0 as f64
            || c.x - c.radius < 0.0
            || c.y + c.radius > self.size.1 as f64
            || c.y - c.radius < 0.0
    }

    pub fn update(&mut self) {
        let mut to_stop: Vec<usize> = self.find_colliding();
        for _ in 0..to_stop.len() {
            let mut c = &mut self.hashgrid.all_elements[to_stop.pop().unwrap()];
            c.dynamic = false;
        }

        let l = self.hashgrid.all_elements.len();
        for i in l - to_stop.len()..l {
            let r = &self.hashgrid.all_elements[i];
            let range = r.get_range(self.hashgrid.cell_dimensions);
            self.hashgrid.insert_element(range, i);
        }

        for c in &mut self.hashgrid.all_elements {
            if c.dynamic {
                c.change_radius(self.rate);
            }
        }
        self.hashgrid.update();
    }
}
