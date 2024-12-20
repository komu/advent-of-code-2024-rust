use crate::directions::CARDINAL_DIRECTIONS;
use crate::grid::Grid;
use crate::vec2::Vec2;
use std::collections::VecDeque;
use std::ops::{Index, Range};
use itertools::iproduct;
use rayon::prelude::*;
type Coordinate = Vec2<i32>;

pub struct ByteGrid<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> ByteGrid<'a> {

    pub fn new(input: &'a str) -> Self {
        let width = input.lines().next().map_or(0, |line| line.len());
        debug_assert!(
            input.lines().all(|line| line.len() == width),
            "All lines must have the same length"
        );
        let height = input.lines().count();

        Self {
            data: input.as_bytes(),
            width,
            height,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn x_range(&self) -> Range<usize> {
        0..self.width
    }

    pub fn y_range(&self) -> Range<usize> {
        0..self.height
    }

    pub fn points(&self) -> impl Iterator<Item = Coordinate> + '_ {
        iproduct!(0..self.height as i32, 0..self.width as i32).map(move |(x,y)| Vec2::new(x, y))
    }

    pub fn par_points(&self) -> impl ParallelIterator<Item = Coordinate> + '_ {
        (0..self.height as i32).into_par_iter()
            .flat_map(move |y| (0..self.width as i32).into_par_iter().map(move |x| Vec2::new(x, y)))
    }

    pub fn find_all(&self, c: u8) -> impl Iterator<Item = Coordinate> + '_ {
        self.points().filter(move |p| self[p] == c)
    }

    pub fn find(&self, c: u8) -> Option<Coordinate> {
        self.points().find(|p| self[p] == c)
    }

    pub fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y).unwrap_or(b' ') as char);
            }
            println!();
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.data[self.index(x, y)])
    }

    pub fn contains(&self, p: &Coordinate) -> bool {
        p.x >= 0 && (p.x as usize) < self.width && p.y >= 0 && (p.y as usize) < self.height
    }

    fn index(&self, x: usize, y: usize) -> usize {
        // add 1 to line lengths for newlines
        y * (self.width + 1) + x
    }
}

impl ByteGrid<'_> {
    pub fn distances_from<T : Fn(u8) -> bool>(&self, start: Coordinate, accept: T) -> Grid<u16> {
        let mut costs = Grid::<u16>::new(self.width, self.height, u16::MAX);
        costs[&start] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            let cost = costs[&u];

            for d in CARDINAL_DIRECTIONS {
                let n = u + d.to_vec();
                if accept(self[&n]) && costs[&n] == u16::MAX {
                    costs[&n] = cost + 1;
                    queue.push_back(n);
                }

            }
        }

        costs
    }
}

impl Index<&Coordinate> for ByteGrid<'_> {
    type Output = u8;

    fn index(&self, index: &Coordinate) -> &Self::Output {
        if self.contains(index) {
            &self.data[self.index(index.x as usize, index.y as usize)]
        } else {
            &0
        }
    }
}
