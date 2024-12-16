use crate::util::vec2::Vec2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

type Path = HashSet<Vec2<i32>>;

#[derive(PartialEq, Eq)]
struct State {
    cost: i32,
    dir: Vec2<i32>,
    heuristic: i32,
    pos: Vec2<i32>,
    path: Path,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_paths(start: Vec2<i32>, end: Vec2<i32>, obstacles: &HashSet<Vec2<i32>>) -> (Vec<Path>, i32) {
    let mut heads = BinaryHeap::new();
    heads.push(State {
        cost: 0,
        heuristic: (end - start).manhattan_distance(),
        pos: start,
        dir: Vec2::new(1, 0),
        path: HashSet::from([start]),
    });
    let mut visited = HashMap::new();
    let mut paths = Vec::new();

    let all_dirs = Vec2::all_dirs();

    let mut path_cost = i32::MAX;

    loop {
        let head = heads.pop().expect("No path");

        visited.insert(head.pos, head.cost);

        if head.cost > path_cost {
            return (paths, path_cost);
        }

        if head.pos == end {
            path_cost = head.cost;
            paths.push(head.path);
            continue;
        }

        for &dir in &all_dirs {
            let pos = head.pos + dir;
            let cost = head.cost + if dir == head.dir { 1 } else { 1001 };

            if obstacles.contains(&pos) {
                continue;
            }


            if head.path.contains(&pos) {
                continue;
            }

            if let Some(&x) = visited.get(&pos) {
                if cost > x + 1000 {
                    continue;
                }
            }

            let mut path = head.path.clone();
            path.insert(pos);


            heads.push(State {
                cost,
                heuristic: cost + (end - start).manhattan_distance(),
                pos,
                dir,
                path,
            });
        }
    }
}

fn part1(start: Vec2<i32>, end: Vec2<i32>, obstacles: &HashSet<Vec2<i32>>) -> i32 {
    let (_, len) = shortest_paths(start, end, &obstacles);
    len
}

fn part2(start: Vec2<i32>, end: Vec2<i32>, obstacles: &HashSet<Vec2<i32>>) -> i32 {
    let (paths, _) = shortest_paths(start, end, &obstacles);

    println!("Num paths found: {}", paths.len());

    HashSet::<Vec2<i32>>::from_iter(
        paths
            .iter()
            .flat_map(|p|
                p.iter().cloned()
            ))
        .len() as i32
}

fn parse_input(input: &str) -> (Vec2<i32>, Vec2<i32>, HashSet<Vec2<i32>>) {
    let mut start = Vec2::zero();
    let mut end = Vec2::zero();
    let mut obstacles = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vec2::new(x as i32, y as i32);
            match c {
                '#' => { obstacles.insert(pos); }
                'S' => start = pos,
                'E' => end = pos,
                _ => ()
            }
        }
    }

    (start, end, obstacles)
}
pub fn day16() {
    let input = fs::read_to_string("inputs/day16.txt").expect("Could not read input");

    let (start, end, obstacles) = parse_input(&input);
    println!("Part 1: {}", part1(start, end, &obstacles));
    println!("Part 2: {}", part2(start, end, &obstacles));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let (start, end, obstacles) = parse_input(&input);
        assert_eq!(7036, part1(start, end, &obstacles));
    }

    #[test]
    fn test_part1_2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let (start, end, obstacles) = parse_input(&input);
        assert_eq!(11048, part1(start, end, &obstacles));
    }

    #[test]
    fn test_part2() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let (start, end, obstacles) = parse_input(&input);
        assert_eq!(45, part2(start, end, &obstacles));
    }

    #[test]
    fn test_part2_2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let (start, end, obstacles) = parse_input(&input);
        assert_eq!(64, part2(start, end, &obstacles));
    }
}