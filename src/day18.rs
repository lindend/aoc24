use crate::util::vec2::Vec2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

#[derive(PartialEq, Eq)]
struct State {
    cost: i32,
    heuristic: i32,
    pos: Vec2<i32>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(start: Vec2<i32>, end: Vec2<i32>, obstacles: &HashSet<Vec2<i32>>, min: Vec2<i32>, max: Vec2<i32>) -> Option<i32> {
    let mut heads = BinaryHeap::new();
    heads.push(State {
        cost: 0,
        heuristic: (end - start).manhattan_distance(),
        pos: start,
    });
    let mut visited = HashSet::new();

    let all_dirs = Vec2::all_dirs();

    loop {
        let head = heads.pop()?;

        if !visited.insert(head.pos) {
            continue;
        }

        if head.pos == end {
            return Some(head.cost);
        }

        for &dir in &all_dirs {
            let pos = head.pos + dir;

            if !pos.in_bounds(min, max) {
                continue;
            }

            if obstacles.contains(&pos) {
                continue;
            }

            let cost = head.cost + 1;


            heads.push(State {
                cost,
                heuristic: cost + (end - start).manhattan_distance(),
                pos,
            });
        }
    }
}

fn part1(falling: &Vec<Vec2<i32>>, num_steps: usize, size: Vec2<i32>) -> i32 {
    let end = size;
    shortest_path(Vec2::zero(), end, &HashSet::from_iter(falling.iter().take(num_steps).cloned()), Vec2::zero(), end).unwrap()
}

fn parse_input(input: &str) -> Vec<Vec2<i32>> {
    input.lines()
        .filter_map(|l| l.split_once(","))
        .map(|(x, y)| Vec2::new(x.parse().unwrap(), y.parse().unwrap()))
        .collect()
}

fn part2(falling: &Vec<Vec2<i32>>, size: Vec2<i32>) -> Vec2<i32> {
    let mut min = 0;
    let mut max = falling.len() - 1;
    loop {
        let i = (max + min) / 2;
        let obstacles = &HashSet::from_iter(falling.iter().take(i).cloned());
        if shortest_path(Vec2::zero(), size, &obstacles, Vec2::zero(), size).is_some() {
            min = max.min(i + 1);
        } else {
            max = min.max(max - 1);
        }
        if max == min {
            return falling[max - 1];
        }
    }
}

pub fn day18() {
    let input = fs::read_to_string("inputs/day18.txt").expect("Could not read input");

    let falling = parse_input(&input);

    println!("Part 1: {}", part1(&falling, 1024, Vec2::new(70, 70)));
    println!("Part 2: {}", part2(&falling, Vec2::new(70, 70)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let falling = parse_input(&input);
        assert_eq!(22, part1(&falling, 12, Vec2::new(6, 6)));
    }

    #[test]
    fn test_p2() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let falling = parse_input(&input);
        assert_eq!(Vec2::new(6, 1), part2(&falling, Vec2::new(6, 6)));
    }
}