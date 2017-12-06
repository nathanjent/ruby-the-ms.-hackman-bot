//! A simple matrix to index into a vector
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix<T> {
    pub m: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            m: Vec::new(),
            width: width,
            height: height,
        }
    }

    pub fn reset_to(&mut self, value: T)
        where T: Copy
    {
        for v in self.m.iter_mut() {
            *v = value;
        }
    }

    /// Get the coordinates for the given index.
    pub fn position(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.height)
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, i: (usize, usize)) -> &T {
        &self.m[i.0 * self.width + i.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut T {
        &mut self.m[i.0 * self.width + i.1]
    }
}

#[cfg(test)]
mod Test {
    use simple_matrix::Matrix;

    #[test]
    fn position_test() {
        let matrix = Matrix {
            m: vec![
                0, 1, 2,
                3, 4, 5,
                6, 7, 8],
            width: 3,
            height: 3,
        };

        assert_eq!(matrix[(0, 0)], matrix[matrix.position(0)]);
        assert_eq!(matrix[(1, 0)], matrix[matrix.position(1)]);
        assert_eq!(matrix[(2, 0)], matrix[matrix.position(2)]);
        assert_eq!(matrix[(0, 1)], matrix[matrix.position(3)]);
        assert_eq!(matrix[(1, 1)], matrix[matrix.position(4)]);
        assert_eq!(matrix[(2, 1)], matrix[matrix.position(5)]);
        assert_eq!(matrix[(0, 2)], matrix[matrix.position(6)]);
        assert_eq!(matrix[(1, 2)], matrix[matrix.position(7)]);
        assert_eq!(matrix[(2, 2)], matrix[matrix.position(8)]);
    }
}
