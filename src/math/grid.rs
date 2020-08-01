pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: Vec::new(),
        }
    }
}

impl<T, I: Into<(usize, usize)>> std::ops::Index<I> for Grid<T> {
    type Output = T;

    fn index(&self, position: I) -> &Self::Output {
        let index = self.position_to_index(position);
        &self.cells[index]
    }
}

impl<T, I: Into<(usize, usize)>> std::ops::IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, position: I) -> &mut Self::Output {
        let index = self.position_to_index(position);
        &mut self.cells[index]
    }
}

impl<T> Grid<T> {
    #[inline]
    fn index_to_position(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.height;
        
        (x, y)
    }

    #[inline]
    fn position_to_index(&self, position: impl Into<(usize, usize)>) -> usize {
        let position = position.into();
        position.0 * self.height + position.1
    }
}
