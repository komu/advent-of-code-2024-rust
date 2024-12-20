use crate::vec2::Vec2;
use std::ops::{Index, IndexMut};

type Coordinate = Vec2<i32>;

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T : Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        Self {
            data: vec![default_value; width * height],
            width,
            height,
        }
    }
}

impl <T> Grid<T> {
    pub fn contains(&self, p: &Coordinate) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl <T> Index<&Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Coordinate) -> &Self::Output {
        assert!(self.contains(index));
        &self.data[self.offset(index.x as usize, index.y as usize)]
    }
}

impl <T> IndexMut<&Coordinate> for Grid<T> {

    fn index_mut(&mut self, index: &Coordinate) -> &mut Self::Output {
        assert!(self.contains(index));
        let offset = self.offset(index.x as usize, index.y as usize);
        &mut self.data[offset]
    }
}