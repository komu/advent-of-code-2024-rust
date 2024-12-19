use itertools::Itertools;
use std::iter::repeat;
use std::ops::Range;
use BlockReservation::*;

advent_of_code::solution!(9);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum BlockReservation {
    FileBlock(u16),
    FreeBlock,
}

struct File {
    id: u16,
    offset: u32,
    blocks: u32,
}

struct Free {
    offset: u32,
    blocks: u32,
}

impl BlockReservation {
    fn id(self: BlockReservation) -> u16 {
        match self {
            FileBlock(id) => id,
            FreeBlock => 0,
        }
    }
}

fn defrag1(buffer: &mut Vec<BlockReservation>) -> u64 {
    let mut empty_indices: Vec<_> = buffer
        .iter()
        .enumerate()
        .rev()
        .filter(|&(_, &v)| v == FreeBlock)
        .map(|(i, _)| i)
        .collect();

    while *empty_indices.last().unwrap() < buffer.len() {
        if let Some(last @ FileBlock(_)) = buffer.pop() {
            buffer[empty_indices.pop().unwrap()] = last;
        }
    }

    buffer
        .iter()
        .enumerate()
        .map(|(i, block)| i as u64 * block.id() as u64)
        .sum()
}

struct FreeBlocks {
    frees: Vec<Free>,
    cleanup_counter: u32,
}

impl FreeBlocks {
    fn new() -> Self {
        FreeBlocks {
            frees: Vec::new(),
            cleanup_counter: 0,
        }
    }

    fn push(&mut self, offset: u32, blocks: u32) {
        self.frees.push(Free { offset, blocks });
    }

    fn try_to_move(&mut self, file: &File) -> Option<u32> {
        let first_free_block = self
            .frees
            .iter_mut()
            .take_while(|b| b.offset < file.offset)
            .find(|b| b.blocks >= file.blocks);

        if let Some(free) = first_free_block {
            let found_offset = free.offset;

            free.offset += file.blocks;
            free.blocks -= file.blocks;

            if free.blocks == 0 {
                self.cleanup_counter += 1;

                if self.cleanup_counter > 100 {
                    self.frees.retain(|it| it.blocks > 0);
                    self.cleanup_counter = 0;
                }
            }

            Some(found_offset)
        } else {
            None
        }
    }
}

fn defrag2(files: &[File], mut frees: FreeBlocks) -> u64 {
    files
        .iter()
        .rev()
        .map(|file| {
            let target_offset = frees.try_to_move(file).unwrap_or(file.offset);
            (file.id as u64) * sum(target_offset..(target_offset + file.blocks))
        })
        .sum()
}

fn sum(range: Range<u32>) -> u64 {
    fn sum_from_zero(n: i64) -> i64 {
        (n - 1) * n / 2
    }

    (sum_from_zero(range.end as i64) - sum_from_zero(range.start as i64)) as u64
}

fn parse(input: &str) -> Vec<(u16, u32, u32)> {
    input
        .chars()
        .map(|c| c.to_digit(10))
        .chunks(2)
        .into_iter()
        .enumerate()
        .map(|(id, mut chunk)| {
            let (file_blocks, free_blocks) = chunk.next_tuple().unwrap();
            (id as u16, file_blocks.unwrap(), free_blocks.unwrap_or(0))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut buffer = Vec::<BlockReservation>::new();

    for (id, blocks, free_blocks) in parse(input) {
        buffer.extend(repeat(FileBlock(id)).take(blocks as usize));
        buffer.extend(repeat(FreeBlock).take(free_blocks as usize));
    }

    Some(defrag1(&mut buffer))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut offset = 0;
    let mut files = Vec::<File>::new();
    let mut frees = FreeBlocks::new();

    for (id, blocks, free_blocks) in parse(input) {
        files.push(File { id, offset, blocks });
        offset += blocks;

        if free_blocks != 0 {
            frees.push(offset, free_blocks);
            offset += free_blocks;
        }
    }

    Some(defrag2(&files, frees))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
