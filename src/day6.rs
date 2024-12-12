use std::collections::{HashMap, HashSet};
use std::fs;

type Pos = (i32, i32);

fn parse_input(input: &str) -> (Pos, HashSet<Pos>, Pos) {
    let lines: Vec<_> = input.lines().collect();
    let mut guard_pos = (0, 0);
    let mut obstacles = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            if cell == '#' {
                obstacles.insert((x, y));
            } else if cell == '^' {
                guard_pos = (x, y);
            }
        }
    }

    (guard_pos, obstacles, (lines[0].len() as i32, lines.len() as i32))
}

fn add((x0, y0): Pos, (x1, y1): Pos) -> Pos {
    (x0 + x1, y0 + y1)
}

fn subtract((x0, y0): Pos, (x1, y1): Pos) -> Pos {
    (x0 - x1, y0 - y1)
}

fn turn_right((dx, dy): Pos) -> Pos {
    (-dy, dx)
}

fn turn_left((dx, dy): Pos) -> Pos {
    (dy, -dx)
}

fn in_bounds((px, py): Pos, (width, height): Pos) -> bool {
    px >= 0 && py >= 0 && px < width && py < height
}

fn part1(guard_pos: Pos, forward: Pos, obstacles: &HashSet<Pos>, size: Pos) -> Option<HashMap<Pos, Vec<Pos>>> {
    let mut visited: HashMap<Pos, Vec<Pos>> = HashMap::new();
    let mut forward = forward;
    visited.insert(guard_pos, vec![forward]);
    let mut current_pos = guard_pos;

    loop {
        let next_pos = add(current_pos, forward);
        if !in_bounds(next_pos, size) {
            return Some(visited);
        }
        if obstacles.contains(&next_pos) {
            forward = turn_right(forward);
        } else {
            current_pos = next_pos;
        }

        if visited.get(&current_pos).map(|dir| dir.contains(&forward)).unwrap_or(false) {
            return None;
        }

        visited.entry(current_pos)
            .and_modify(|v| v.push(forward))
            .or_insert(vec![forward]);
    }
}

fn backtrack(pos: Pos, forward: Pos, obstacles: &HashSet<Pos>, size: Pos, visited: &mut HashMap<Pos, Vec<Pos>>) {
    let mut forward = forward;
    let mut current_pos = pos;

    loop {
        if visited.get(&current_pos).map(|p| p.contains(&forward)).unwrap_or(false) {
            return;
        }

        visited.entry(current_pos)
            .and_modify(|v| v.push(forward))
            .or_insert(vec![forward]);

        let next_pos = subtract(current_pos, forward);
        if !in_bounds(next_pos, size) {
            return;
        }

        if obstacles.contains(&next_pos) {
            forward = turn_left(forward);
        } else {
            current_pos = next_pos;
        }
    }
}

fn part2(guard_pos: Pos, obstacles: &HashSet<Pos>, size: Pos) -> i32 {
    let mut new_obstacles = HashSet::new();
    let mut visited: HashMap<Pos, Vec<Pos>> = HashMap::new();

    let mut forward = (0, -1);
    let mut current_pos = guard_pos;

    backtrack(current_pos, forward, &obstacles, size, &mut visited);

    loop {
        visited.entry(current_pos)
            .and_modify(|v| v.push(forward))
            .or_insert(vec![forward]);

        let next_pos = add(current_pos, forward);
        if !in_bounds(next_pos, size) {
            return new_obstacles.len() as i32;
        }

        if obstacles.contains(&next_pos) {
            forward = turn_right(forward);
            backtrack(current_pos, forward, &obstacles, size, &mut visited);
        } else {
            current_pos = next_pos;
        }

        if visited.get(&current_pos).map(|dir| dir.contains(&turn_right(forward))).unwrap_or(false) {
            let new_obstacle = add(current_pos, forward);
            if new_obstacle != guard_pos {
                new_obstacles.insert(new_obstacle);
            }
        }
    }
}

fn part2_v2(guard_pos: Pos, obstacles: &HashSet<Pos>, size: Pos) -> i32 {
    let mut new_obstacles = HashSet::new();
    let mut obstacles = obstacles.clone();

    let mut forward = (0, -1);
    let mut current_pos = guard_pos;

    loop {
        let next_pos = add(current_pos, forward);
        if !in_bounds(next_pos, size) {
            return new_obstacles.len() as i32;
        }

        if obstacles.contains(&next_pos) {
            forward = turn_right(forward);
        } else {
            current_pos = next_pos;
        }

        let test_obstacle = add(current_pos, forward);
        if test_obstacle != guard_pos && obstacles.insert(test_obstacle) {
            if part1(guard_pos, (0, -1), &obstacles, size) == None {
                new_obstacles.insert(test_obstacle);
            }
            obstacles.remove(&test_obstacle);
        }
    }
}

pub fn day6() {
    let input = fs::read_to_string("inputs/day6.txt")
        .expect("Could not read input file");

    let (guard_pos, obstacles, size) = parse_input(&input);

    println!("Part 1: {}", part1(guard_pos, (0, -1), &obstacles, size).unwrap().len());
    println!("Part 2: {}", part2_v2(guard_pos, &obstacles, size));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let (guard_pos, obstacles, size) = parse_input(&input);
        assert_eq!(41, part1(guard_pos, (0, -1), &obstacles, size).unwrap().len());
    }

    #[test]
    pub fn test_p2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let (guard_pos, obstacles, size) = parse_input(&input);
        assert_eq!(6, part2_v2(guard_pos, &obstacles, size));
    }
}