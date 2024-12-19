use itertools::Itertools;
use std::ops::Range;

advent_of_code::solution!(9);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum BlockReservation {
    File(u16),
    Free,
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

impl Free {
    fn consume(&mut self, blocks: u32) {
        self.offset += blocks;
        self.blocks -= blocks;
    }
}

impl BlockReservation {
    fn id(self: BlockReservation) -> u16 {
        match self {
            BlockReservation::File(id) => id,
            BlockReservation::Free => 0,
        }
    }

    fn is_free(self: BlockReservation) -> bool {
        match self {
            BlockReservation::File(_) => false,
            BlockReservation::Free => true,
        }
    }
}

fn defrag1(buffer: &mut Vec<BlockReservation>) -> u64 {
    let mut empty_indices: Vec<_> = buffer
        .iter()
        .enumerate()
        .rev()
        .filter(|&(_, v)| v.is_free())
        .map(|(i, _)| i)
        .collect();

    while *empty_indices.last().unwrap() < buffer.len() {
        if let Some(last @ BlockReservation::File(_)) = buffer.pop() {
            buffer[empty_indices.pop().unwrap()] = last;
        }
    }

    buffer
        .iter()
        .enumerate()
        .map(|(i, block)| i as u64 * block.id() as u64)
        .sum()
}

fn defrag2(files: &[File], mut frees: Vec<Free>) -> u64 {
    let mut cleanup_counter = 0;
    let mut checksum: u64 = 0;

    for file in files.iter().rev() {
        let free = find_first_free_block(&mut frees, file);
        let target_offset = free.as_ref().map(|it| it.offset).unwrap_or(file.offset);

        if let Some(free) = free {
            free.consume(file.blocks);
            if free.blocks == 0 {
                cleanup_counter += 1;
            }
        }

        if cleanup_counter >= 250 {
            frees.retain(|it| it.blocks > 0);
            cleanup_counter = 0;
        }

        checksum += (file.id as u64) * sum(target_offset ..(target_offset + file.blocks))
    }

    checksum
}

fn sum(range: Range<u32>) -> u64 {
    fn sum_from_zero(n: i64) -> i64 {
        (n - 1) * n / 2
    }

    (sum_from_zero(range.end as i64) - sum_from_zero(range.start as i64)) as u64
}

fn find_first_free_block<'a>(frees: &'a mut [Free], file: &File) -> Option<&'a mut Free> {
    frees
        .iter_mut()
        .take_while(|it| it.offset < file.offset)
        .find(|it| it.blocks >= file.blocks)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut buffer = Vec::<BlockReservation>::new();

    for (id, chunk) in input.chars().map(|c| c.to_digit(10)).chunks(2).into_iter().enumerate() {
        let chunk = chunk.collect::<Vec<_>>();
        let file_blocks = chunk[0].unwrap();
        let free_block = chunk[1].unwrap_or(0);

        for _ in 0..file_blocks {
            buffer.push(BlockReservation::File(id as u16));
        }
        for _ in 0..free_block {
            buffer.push(BlockReservation::Free);
        }
    }

    Some(defrag1(&mut buffer))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut offset = 0;
    let mut files = Vec::<File>::new();
    let mut frees = Vec::<Free>::new();

    for (id, chunk) in input.chars().map(|c| c.to_digit(10)).chunks(2).into_iter().enumerate() {
        let chunk = chunk.collect::<Vec<_>>();
        let file_blocks = chunk[0].unwrap();
        let free_blocks = chunk[1].unwrap_or(0);

        files.push(File { id: id as u16, offset, blocks: file_blocks });
        offset += file_blocks;

        if free_blocks != 0 {
            frees.push(Free { offset, blocks: free_blocks });
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
