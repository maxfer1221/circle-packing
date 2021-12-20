pub struct HashGrid<T> {
    dimensions: (usize, usize),
    elements: Vec<Vec<Vec<T>>>,
}

impl<T> HashGrid<T> {
    pub fn elements_at(&self, x: usize, y: usize) -> &Vec<T> {
        &self.elements[x][y]
    }
    fn insert_element(&mut self, x: usize, y: usize, el: T) {
        self.elements[x][y].push(el);
    }
}
