use std::fs;

#[derive(Clone, Debug)]
pub enum DiskBlock {
    Allocated(u64),
    Free,
}

type DiskChunk = (DiskBlock, u64, u64);

fn checksum_disk(disk: &Vec<DiskBlock>) -> u64 {
    disk
        .iter()
        .enumerate()
        .map(|(i, b)| match b {
            DiskBlock::Allocated(n) => n * (i as u64),
            DiskBlock::Free => 0
        })
        .sum()
}

pub fn part1(input: &Vec<DiskBlock>) -> u64 {
    let mut i = 0;
    let mut j = input.len() - 1;
    let mut compressed_disk = Vec::new();
    while i <= j {
        compressed_disk.push(
            match input[i] {
                DiskBlock::Allocated(i) => DiskBlock::Allocated(i),
                DiskBlock::Free => loop {
                    if let DiskBlock::Allocated(n) = input[j] {
                        j -= 1;
                        break DiskBlock::Allocated(n);
                    }
                    j -= 1;
                    if j <= i {
                        break DiskBlock::Free;
                    }
                }
            });
        i += 1;
    }
    checksum_disk(&compressed_disk)
}

pub fn part2(input: &Vec<DiskChunk>) -> u64 {
    let mut compressed = input.clone();

    let mut j = input.len() - 1;
    while j > 0 {
        let (old_block, old_size, old_free) = (&compressed[j]).clone();
        if let DiskBlock::Free = old_block {
            j -= 1;
            continue;
        }

        let first_free = compressed.iter().enumerate().find(|(_, (_, _, free))| *free >= old_size);
        if let Some((i, find_res)) = first_free {
            let (block, n, free) = find_res.clone();
            if i < j {
                compressed[i] = (block.clone(), n, 0);
                compressed[j] = (DiskBlock::Free, 0, old_size + old_free);
                compressed.insert(i + 1, (old_block.clone(), old_size, free - old_size));
            } else {
                j -= 1;
            }
        } else {
            j -= 1;
        }
    }

    let flattened = compressed.iter()
        .flat_map(|(block, size, free)| std::iter::repeat_n(block.clone(), *size as usize).chain(std::iter::repeat_n(DiskBlock::Free, *free as usize)))
        .collect();

    checksum_disk(&flattened)
}

pub fn parse_input1(input: &str) -> Vec<DiskBlock> {
    input.chars()
        .map(|c| c.to_digit(10).expect("Could not parse digit"))
        .enumerate()
        .flat_map(|(i, d)| std::iter::repeat_n(if i % 2 == 0 { DiskBlock::Allocated((i / 2) as u64) } else { DiskBlock::Free }, d as usize))
        .collect()
}

pub fn parse_input2(input: &str) -> Vec<DiskChunk> {
    let mut input = input.chars()
        .map(|c| c.to_digit(10).expect("Could not parse digit") as u64)
        .enumerate();

    let mut res = Vec::new();

    loop {
        let next = input.next();
        if next.is_none() {
            break;
        }

        let (i, allocated) = next.unwrap();
        let (_, free) = input.next().unwrap_or((0, 0));
        res.push((DiskBlock::Allocated((i / 2) as u64), allocated, free));
    }

    res
}

pub fn day9() {
    let input = fs::read_to_string("inputs/day9.txt")
        .expect("Could not load input");
    let disk1 = parse_input1(&input);
    let disk2 = parse_input2(&input);

    println!("Part 1: {}", part1(&disk1));
    println!("Part 2: {}", part2(&disk2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = "2333133121414131402";
        let disk = parse_input1(&input);
        assert_eq!(1928, part1(&disk));
    }

    #[test]
    pub fn test_p2() {
        let input = "2333133121414131402";
        let disk = parse_input2(&input);
        assert_eq!(2858, part2(&disk));
    }
}