use crate::circles::Circle;

pub struct Scene {
    pub circles: Vec<Circle>,
    dynamic: Vec<Circle>,
    rate: f64,
    size: (f64, f64),
    //hash_grid: HashGrid,
}

impl Scene {
    pub fn new(dims: (u32, u32), rate: f64, c: Vec<Circle>) -> Self {
        Scene {
            circles: c.clone(),
            dynamic: c,
            rate: rate,
            size: (dims.0 as f64, dims.1 as f64),
        }
    }

    fn add_circle(&mut self, c: Circle) {
        self.circles.push(c);
    }

    fn find_collisions(&mut self) -> Vec::<Circle> {
        let mut to_remove = Vec::<Circle>::new();
        for circle in &self.dynamic {
            if circle.colliding_any(&self.circles) {
                to_remove.push(circle.copy());
            }
        }
        to_remove
    }

    fn update(&mut self) {
        let to_remove: Vec<Circle> = self.find_collisions();
        for circle in to_remove {
            self.dynamic.retain(|x| *x != circle);
        }
        for dynamic in &mut self.dynamic {
            dynamic.change_radius(self.rate);
        }
    }
}