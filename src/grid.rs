use crate::vec2::Vec2;
use itertools::iproduct;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

type Coordinate = Vec2<i32>;

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        Self {
            data: vec![default_value; width * height],
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn contains(&self, p: &Coordinate) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }

    pub fn get(&self, p: &Coordinate) -> Option<&T> {
        if self.contains(p) {
            Some(&self.data[self.offset(p.x as usize, p.y as usize)])
        } else {
            None
        }
    }

    pub fn points(&self) -> impl Iterator<Item = Coordinate> + '_ {
        iproduct!(0..self.height as i32, 0..self.width as i32).map(move |(x,y)| Vec2::new(x, y))
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl<T: Debug> Grid<T> {
    pub fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{:?} ", self.get(&Vec2::new(x as i32, y as i32)).unwrap());
            }
            println!();
        }
    }
}
impl<T: Eq + Clone> Grid<T> {
    pub fn flood<P: Fn(&Coordinate) -> bool>(
        &mut self,
        initial: &Coordinate,
        value: T,
        ds: &[Vec2<i32>],
        uninitialized: T,
        accept: P,
    ) {
        let mut stack = vec![*initial];

        while let Some(p) = stack.pop() {
            if self.contains(&p) && self[&p] == uninitialized && accept(&p) {
                self[&p] = value.clone();
                stack.extend(ds.iter().map(|&d| p + d));
            }
        }
    }
}

impl<T> Index<&Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Coordinate) -> &Self::Output {
        assert!(self.contains(index));
        &self.data[self.offset(index.x as usize, index.y as usize)]
    }
}

impl<T> IndexMut<&Coordinate> for Grid<T> {
    fn index_mut(&mut self, index: &Coordinate) -> &mut Self::Output {
        assert!(self.contains(index));
        let offset = self.offset(index.x as usize, index.y as usize);
        &mut self.data[offset]
    }
}
