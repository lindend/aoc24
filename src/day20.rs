use crate::util::vec2::Vec2;
use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(input: &str) -> Vec<Vec2<i32>> {
    let mut track = HashSet::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Vec2::new(x as i32, y as i32);
            match c {
                '.' => { track.insert(coord); }
                'S' => start = Some(coord),
                'E' => {
                    end = Some(coord);
                    track.insert(coord);
                }
                _ => {}
            }
        }
    }

    let mut res = Vec::new();

    res.push(start.unwrap());

    let dirs = Vec2::all_dirs();

    let mut last = start.unwrap();
    let mut current = last;

    while current != end.unwrap() {
        for &dir in &dirs {
            let p = current + dir;
            if p == last {
                continue;
            }

            if track.contains(&p) {
                res.push(p);
                last = current;
                current = p;
                break;
            }
        }
    }
    res.push(end.unwrap());


    res
}

fn part1(track: &Vec<Vec2<i32>>, min_shortcut: i32, cheat_length: i32) -> i32 {
    let trackmap: &HashMap<Vec2<i32>, i32> = &HashMap::from_iter(
        track.iter()
            .copied()
            .enumerate()
            .map(|(i, t)| (t, i as i32))
    );

    let shortcuts: Vec<_> = (-cheat_length..=cheat_length)
        .flat_map(|xl|
            (-cheat_length..=cheat_length).map(move |yl|
                Vec2::new(xl, yl)
            )
        )
        .filter(|v| v.manhattan_distance() <= cheat_length)
        .collect();

    let cheats: HashSet<_> = HashSet::from_iter(track.iter()
        .enumerate()
        .flat_map(|(i, &pos)| {
            let i = i as i32;
            shortcuts.iter()
                .map(move |&s| (pos, pos + s, trackmap.get(&(pos + s)).unwrap_or(&i) - i - s.manhattan_distance()))
                .filter(move |(_, _, c)| *c >= min_shortcut)
        }
        ));


    cheats.len() as i32
}

pub fn day20() {
    let input = fs::read_to_string("inputs/day20.txt").expect("Could not read input");

    let track = parse_input(&input);

    println!("Part 1: {}", part1(&track, 100, 2));
    println!("Part 2: {}", part1(&track, 100, 20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        let track = parse_input(&input);
        assert_eq!(44, part1(&track, 2, 2));
    }

    #[test]
    fn test_p2() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        let track = parse_input(&input);
        assert_eq!(285, part1(&track, 50, 20));
    }
}