use crate::vec2::Vec2;
use std::ops::Index;

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

    pub fn points(&self) -> impl Iterator<Item = Coordinate> + '_ {
        (0..self.height as i32)
            .flat_map(move |y| (0..self.width as i32).map(move |x| Vec2::new(x, y)))
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
