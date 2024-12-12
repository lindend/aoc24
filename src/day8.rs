use crate::util::vec2::Vec2;
use std::collections::{HashMap, HashSet};
use std::fs;

fn part1(towers: &HashMap<char, Vec<Vec2<i32>>>, size: Vec2<i32>) -> i32 {
    let size = size - Vec2::new(1, 1);
    let mut antinodes = HashSet::new();
    for (c, ts) in towers {
        for i in 0..ts.len() {
            for j in i + 1..ts.len() {
                let t0 = ts[i];
                let t1 = ts[j];
                let delta = t1 - t0;
                let antinode_0 = t0 - delta;
                let antinode_1 = t1 + delta;
                if antinode_0.in_bounds(Vec2::zero(), size) {
                    antinodes.insert(antinode_0);
                }

                if antinode_1.in_bounds(Vec2::zero(), size) {
                    antinodes.insert(antinode_1);
                }
            }
        }
    }

    antinodes.len() as i32
}

fn add_antinodes(origin: Vec2<i32>, delta: Vec2<i32>, antinodes: &mut HashSet<Vec2<i32>>, size: Vec2<i32>) {
    let mut freq = 0;
    loop {
        let antinode = origin + delta * freq;
        if antinode.in_bounds(Vec2::zero(), size) {
            antinodes.insert(antinode);
        } else {
            return;
        }
        freq = freq + 1;
    }
}

fn part2(towers: &HashMap<char, Vec<Vec2<i32>>>, size: Vec2<i32>) -> i32 {
    let size = size - Vec2::new(1, 1);
    let mut antinodes = HashSet::new();
    for (c, ts) in towers {
        for i in 0..ts.len() {
            for j in i + 1..ts.len() {
                let t0 = ts[i];
                let t1 = ts[j];
                let delta = t1 - t0;
                add_antinodes(t0, delta * -1, &mut antinodes, size);
                add_antinodes(t1, delta, &mut antinodes, size);
            }
        }
    }

    antinodes.len() as i32
}

fn parse_input(input: &str) -> (HashMap<char, Vec<Vec2<i32>>>, Vec2<i32>) {
    let mut result = HashMap::new();
    let input: Vec<_> = input.trim().lines().collect();
    let chars = input
        .iter()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate()
            .filter(|(_, c)| *c != '.')
            .map(move |(x, c)| (c, Vec2::new(x as i32, y as i32))))
        .flatten();

    for (c, pos) in chars {
        result.entry(c)
            .and_modify(|e: &mut Vec<Vec2<i32>>| e.push(pos))
            .or_insert(vec![pos]);
    }
    let size = Vec2::new(input[0].len() as i32, input.len() as i32);

    (result, size)
}

pub fn day8() {
    let input = fs::read_to_string("inputs/day8.txt")
        .expect("Could not load input file");

    let (towers, size) = parse_input(&input);

    println!("Part 1: {}", part1(&towers, size));
    println!("Part 2: {}", part2(&towers, size));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let (towers, size) = parse_input(&input);
        assert_eq!(14, part1(&towers, size));
    }

    #[test]
    pub fn test_p2() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let (towers, size) = parse_input(&input);
        assert_eq!(34, part2(&towers, size));
    }
}