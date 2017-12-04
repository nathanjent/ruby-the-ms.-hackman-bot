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
