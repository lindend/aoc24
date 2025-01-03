﻿// use crate::util::vec2::Vec2;
// use std::cmp::Ordering;
// use std::collections::{BinaryHeap, HashMap, HashSet};
// use std::fmt::{Display, Formatter};
// use std::fs;

// #[derive(Clone, PartialEq, Eq)]
// #[derive(Hash)]
// enum Input {
//     Left,
//     Right,
//     Up,
//     Down,
//     A,
// }

// impl Input {
//     fn from_vec(v: Vec2<i32>) -> Self {
//         match v {
//             Vec2 { x: -1, y: 0 } => Input::Left,
//             Vec2 { x: 1, y: 0 } => Input::Right,
//             Vec2 { x: 0, y: -1 } => Input::Up,
//             Vec2 { x: 0, y: 1 } => Input::Down,
//             Vec2 { x: 0, y: 0 } => Input::A,
//             _ => panic!("Invalid delta"),
//         }
//     }
// }

// impl Display for Input {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Input::Left => write!(f, "<"),
//             Input::Right => write!(f, ">"),
//             Input::Up => write!(f, "^"),
//             Input::Down => write!(f, "v"),
//             Input::A => write!(f, "A"),
//         }
//     }
// }

// #[derive(PartialEq, Eq)]
// struct State<'a, T: Clone> {
//     cost: i32,
//     pos: Vec2<i32>,
//     prev_pos: Vec2<i32>,
//     targets: &'a [Vec2<i32>],
//     path: Vec<Vec2<i32>>,
// }

// impl<'a, T: Eq + PartialEq + Clone> Ord for State<'a, T> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//     }
// }

// impl<'a, T: Eq + PartialEq + Clone> PartialOrd for State<'a, T> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// fn shortest_path<T>(
//     start: Vec2<i32>,
//     targets: &Vec<Vec2<i32>>,
//     obstacles: &HashSet<Vec2<i32>>,
//     cost_fn: fn(Vec2<i32>, Vec2<i32>) -> i32,
// ) -> i32 {
//     let mut heads = BinaryHeap::new();
//     heads.push(State {
//         cost: 0,
//         pos: start,
//         prev_pos: start,
//         targets: targets.as_slice(),
//         path: Vec::new(),
//     });

//     let mut visited = HashMap::new();

//     let all_dirs = Vec2::all_dirs();

//     loop {
//         let head = heads.pop().expect("No path");

//         visited.insert(head.pos, head.cost);

//         let mut targets = head.targets;

//         assert!(!targets.is_empty());

//         let mut cost = head.cost;
//         let mut path = head.path.clone();

//         if head.pos == targets[0] {
//             cost += 1;
//             path.push(Vec2::zero());
//             targets = &targets[1..];
//         }

//         if targets.is_empty() {
//             return head.cost
//         }

//         for &dir in &all_dirs {
//             let pos = head.pos + dir;
//             if pos == head.prev_pos {
//                 continue;
//             }

//             let cost = cost + cost_fn(head.pos, pos);

//             if obstacles.contains(&pos) {
//                 continue;
//             }
//             let mut path = path.clone();

//             path.push(dir);

//             heads.push(State {
//                 cost,
//                 pos,
//                 prev_pos: head.pos,
//                 targets,
//                 path,
//             });
//         }
//     }
// }

// trait Controller {

// }

// #[derive(Eq, PartialEq, Hash, Clone)]
// struct Keypad {
//     current_pos: Vec2<i32>,
//     buttons: Vec<Vec2<i32>>,
//     controller: Option<Box<Keypad>>
// }

// impl Keypad {
//     fn navigate(&mut self, targets: &Vec<Vec2<i32>>) -> i32 {
//         let pc = shortest_path(self.current_pos, targets, obstacles, cost_fn, cost_context, cost_state)
//     }
// }

// fn keypad_sequences(code: &str, memo: &mut HashMap<(Vec2<i32>, Input, i32), i32>) -> (i32, Vec<Vec2<i32>>) {
//     let targets = code.chars().map(|c| match c {
//         '7' => Vec2::new(0, 0),
//         '8' => Vec2::new(1, 0),
//         '9' => Vec2::new(2, 0),
//         '4' => Vec2::new(0, 1),
//         '5' => Vec2::new(1, 1),
//         '6' => Vec2::new(2, 1),
//         '1' => Vec2::new(0, 2),
//         '2' => Vec2::new(1, 2),
//         '3' => Vec2::new(2, 2),
//         '0' => Vec2::new(1, 3),
//         'A' => Vec2::new(2, 3),
//         _ => panic!("Invalid input `{c}`"),
//     }).collect();

//     let mut obstacles = HashSet::new();
//     obstacles.insert(Vec2::new(0, 3));
//     shortest_path(
//         Vec2::new(2, 3),
//         &targets,
//         &obstacles,
//         |from, to, &(kp0_state, kp1_state), context|
//             dir_pad_sequences(kp0_state, kp1_state, to - from, true, context),
//         memo,
//         (Vec2::new(2, 0), Vec2::new(2, 0)),
//     )
// }

// fn dir_pad_sequences(kp0_start: Vec2<i32>, kp1_start: Vec2<i32>, delta: Vec2<i32>, recurse: bool, memo: &mut HashMap<(Vec2<i32>, Input, i32), i32>) -> (i32, (Vec2<i32>, Vec2<i32>)) {
//     let input = Input::from_vec(delta);
//     let level = if recurse { 1 } else { 2 };
//     let target = match input {
//         Input::Up => Vec2::new(1, 0),
//         Input::A => Vec2::new(2, 0),
//         Input::Left => Vec2::new(0, 1),
//         Input::Down => Vec2::new(1, 1),
//         Input::Right => Vec2::new(2, 1),
//     };

//     // if let Some(len) = memo.get(&(kp0_start, kp1_start, input, level)) {
//     //     return (*len, target);
//     // }

//     let mut obstacles = HashSet::new();
//     obstacles.insert(Vec2::zero());

//     let start = if level == 1 {
//         kp0_start
//     } else {
//         kp1_start
//     };

//     let (c, _) = shortest_path(
//         start,
//         &vec![target],
//         &obstacles,
//         if recurse {
//             |from: Vec2<i32>, to, &(kp0_start, kp1_start), context| dir_pad_sequences(kp0_start, kp1_start, to - from, false, context)
//         } else {
//             |_, _, &state, _| (1, state)
//         },
//         memo,
//         (kp0_start, kp1_start),
//     );
//     (c, (kp0_start, kp1_start))
//     // memo.insert((delta, level), s);
// }

// fn part1(codes: &Vec<&str>) -> i64 {
//     let mut total_complexity = 0;
//     let mut memo = HashMap::new();
//     for &code in codes {
//         let (code_len, path) = keypad_sequences(code, &mut memo);
//         let path: Vec<_> = path.iter().map(|&p| Input::from_vec(p).to_string()).collect();
//         let path = path.join("");

//         println!("Found sequence {code} len {code_len} {path}");

//         let num_code: i64 = code[..code.len() - 1].parse().expect("Could not parse numeric part of code");
//         let complexity = code_len as i64 * num_code;
//         total_complexity += complexity;
//     }
//     // Too high: 158428

//     total_complexity
// }

// pub fn day21() {
//     let input = fs::read_to_string("inputs/day21.txt")
//         .expect("Could not load input");

//     let codes = input.lines().collect();

//     println!("Part 1: {}", part1(&codes));
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_p1() {
//         let input = "029A
// 980A
// 179A
// 456A
// 379A".lines();
//         let res = part1(&input.collect());
//         assert_eq!(126384, res);
//     }
// }
