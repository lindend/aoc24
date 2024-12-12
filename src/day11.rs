use std::collections::HashMap;
use std::fs;

fn split_stone(stone: u64) -> Vec<u64> {
    let str = stone.to_string();
    let (left, right) = str.split_at(str.len() / 2);
    vec![left.parse().unwrap(), right.parse().unwrap()]
}
fn split_stone_2(stone: u64) -> (u64, u64) {
    let str = stone.to_string();
    let (left, right) = str.split_at(str.len() / 2);
    (left.parse().unwrap(), right.parse().unwrap())
}
fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones.iter()
        .flat_map(|stone| match stone {
            0 => vec![1],
            n => if n.ilog10() % 2 == 1 { split_stone(*stone) } else { vec![stone * 2024] },
        })
        .collect()
}


fn blink_stone(stone: u64, max_level: i32, level: i32, memoize: &mut HashMap<(u64, i32), u64>) -> u64 {
    if level >= max_level {
        return 1;
    }
    let mut memoize = memoize;

    if !memoize.contains_key(&(stone, level)) {
        let value = match stone {
            0 => blink_stone(1, max_level, level + 1, &mut memoize),
            _ => if stone.ilog10() % 2 == 1 {
                let (l, r) = split_stone_2(stone);
                blink_stone(l, max_level, level + 1, &mut memoize) + blink_stone(r, max_level, level + 1, &mut memoize)
            } else {
                blink_stone(stone * 2024, max_level, level + 1, &mut memoize)
            }
        };
        memoize.insert((stone, level), value);
        value
    } else {
        *memoize.get(&(stone, level)).unwrap()
    }
}

fn part1(stones: &Vec<u64>) -> i32 {
    let mut stones = stones.clone();
    for i in 0..25 {
        stones = blink(&stones);
    }
    stones.len() as i32
}

fn part2(stones: &Vec<u64>, max_level: i32) -> u64 {
    stones.iter()
        .map(|&s| blink_stone(s, max_level, 0, &mut HashMap::new()))
        .sum()
}

fn parse_input(input: &str) -> Vec<u64> {
    input.split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect()
}
pub fn day11() {
    let input = fs::read_to_string("inputs/day11.txt").expect("Could not read input file");
    let stones = parse_input(&input);

    println!("Part 1: {}", part1(&stones));
    println!("Part 1 (2): {}", part2(&stones, 25));
    println!("Part 2: {}", part2(&stones, 75));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        let stones = parse_input(&input);
        assert_eq!(55312, part1(&stones));
    }
}