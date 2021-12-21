use std::{fmt::Debug, ops::Range};

#[derive(Debug)]
pub struct HashGrid<T: std::fmt::Debug> {
    pub all_elements: Vec<T>,
    pub dimensions: (f64, f64),
    pub cell_dimensions: (f64, f64),
    pub cell_count: (usize, usize),
    elements: Vec<Vec<Vec<usize>>>,
}

pub trait HashGridHelper {
    fn get_range(&self, c_d: (f64, f64)) -> (Range<usize>, Range<usize>);
}

pub trait Dynamic {
    fn is_dynamic(&self) -> bool;
}

pub trait Indexed {
    fn index(&self) -> usize;
}

impl<T: std::fmt::Debug + Dynamic + Indexed + HashGridHelper> HashGrid<T> {
    pub fn new(d: (u32, u32), cell_d: (f64, f64)) -> Self {
        let c_count = (
            (d.0 as f64 / cell_d.0).ceil() as usize + 1,
            (d.1 as f64 / cell_d.1).ceil() as usize + 1,
        );
        let mut elements = Vec::<Vec<Vec<usize>>>::with_capacity(c_count.0);
        for i in 0..c_count.0 {
            elements.push(Vec::<Vec<usize>>::with_capacity(c_count.1));
            for _ in 0..c_count.1 {
                elements[i].push(Vec::<usize>::new());
            }
        }
        HashGrid {
            all_elements: Vec::new(),
            dimensions: (d.0 as f64, d.1 as f64),
            cell_dimensions: cell_d,
            cell_count: c_count,
            elements,
        }
    }

    pub fn elements_from_range(&self, r: (Range<usize>, Range<usize>)) -> Vec<&T> {
        let mut all = Vec::<&T>::new();
        for i in r.0 {
            for j in r.1.clone() {
                let indeces = &self.elements[i][j];
                for x in indeces {
                    all.push(&self.all_elements[*x]);
                }
            }
        }
        all
    }

    pub fn insert_element(&mut self, r: (Range<usize>, Range<usize>), el: usize) {
        for i in r.0 {
            for j in r.1.clone() {
                self.elements[i][j].push(el);
            }
        }
    }

    pub fn update(&mut self) {
        let mut to_update = Vec::<((Range<usize>, Range<usize>), usize)>::new();
        for c in &mut self.all_elements {
            if c.is_dynamic() {
                let c_range = c.get_range(self.cell_dimensions);
                for i in c_range.0.clone() {
                    let outer = &mut self.elements[i];
                    for j in c_range.1.clone() {
                        let inner = &mut outer[j];
                        inner.retain(|x| *x != c.index());
                    }
                }
                to_update.push((c_range, c.index()));
            }
        }
        for el in to_update {
            self.insert_element(el.0, el.1);
        }
    }
}
