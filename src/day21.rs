use crate::util::vec2::Vec2;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::{fs, iter};

#[derive(Clone, PartialEq, Eq)]
enum Input {
    Left,
    Right,
    Up,
    Down,
    A,
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Left => write!(f, "<"),
            Input::Right => write!(f, ">"),
            Input::Up => write!(f, "^"),
            Input::Down => write!(f, "v"),
            Input::A => write!(f, "A"),
        }
    }
}

fn repeat<T: Clone>(num_times: i32, when_positive: T, when_negative: T) -> Vec<T> {
    if num_times > 0 {
        iter::repeat(when_positive)
    } else {
        iter::repeat(when_negative)
    }
    .take(num_times.abs() as usize)
    .collect()
}

fn l_r_permutations<T: Clone>(left: &[Vec<T>], right: &[Vec<T>]) -> Vec<Vec<T>> {
    assert_eq!(left.len(), right.len());
    assert_ne!(left.len(), 0);

    if left.len() == 1 {
        return vec![left[0].clone(), right[0].clone()];
    }
    let l = &left[0];
    let r = &right[0];

    let perms = l_r_permutations(&left[1..], &right[1..]);

    let ls = perms.iter().map(|p| {
        let mut v = l.clone();
        v.extend_from_slice(p);
        v
    });
    let rs = perms.iter().map(|p| {
        let mut v = r.clone();
        v.extend_from_slice(p);
        v
    });

    ls.chain(rs).collect()
}

fn input_sequences(targets: &Vec<Vec2<i32>>, pos: Vec2<i32>, bad: Vec2<i32>) -> Vec<Input> {
    let mut current = pos;

    let mut inputs: Vec<Input> = Vec::new();

    for &target in targets {
        let delta = target - current;

        let x_moves = repeat(delta.x, Input::Right, Input::Left);
        let y_moves = repeat(delta.y, Input::Down, Input::Up);

        if current.y == bad.y && target.x == bad.x {
            inputs.extend(y_moves.iter().cloned());
            inputs.extend(x_moves.iter().cloned());
        } else {
            inputs.extend(x_moves.iter().cloned());
            inputs.extend(y_moves.iter().cloned());
        };

        inputs.push(Input::A);
        current = target;
    }

    inputs
}

fn keypad_sequences(code: &str) -> Vec<Input> {
    let targets = code.chars().map(|c| match c {
        '7' => Vec2::new(0, 0),
        '8' => Vec2::new(1, 0),
        '9' => Vec2::new(2, 0),
        '4' => Vec2::new(0, 1),
        '5' => Vec2::new(1, 1),
        '6' => Vec2::new(2, 1),
        '1' => Vec2::new(0, 2),
        '2' => Vec2::new(1, 2),
        '3' => Vec2::new(2, 2),
        '0' => Vec2::new(1, 3),
        'A' => Vec2::new(2, 3),
        _ => panic!("Invalid input `{c}`"),
    });

    input_sequences(&targets.collect(), Vec2::new(2, 3), Vec2::new(0, 3))
}

fn dir_pad_sequences(code: &Vec<Input>) -> Vec<Input> {
    let targets = code.iter().map(|c| match c {
        Input::Up => Vec2::new(1, 0),
        Input::A => Vec2::new(2, 0),
        Input::Left => Vec2::new(0, 1),
        Input::Down => Vec2::new(1, 1),
        Input::Right => Vec2::new(2, 1),
    });

    input_sequences(&targets.collect(), Vec2::new(2, 0), Vec2::new(0, 0))
}

fn part1(codes: &Vec<&str>) -> i64 {
    let mut total_complexity = 0;
    for &code in codes {
        let ks = keypad_sequences(code);
        let dpr1s: Vec<_> = dir_pad_sequences(&ks);
        let dpms: Vec<_> = dir_pad_sequences(&dpr1s);

        println!(
            "{code}: {}",
            dpms.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        );

        let num_code: i64 = code[..code.len() - 1]
            .parse()
            .expect("Could not parse numeric part of code");
        let complexity = dpms.len() as i64 * num_code;
        total_complexity += complexity;
    }
    // Too high: 158428

    total_complexity
}

pub fn day21() {
    let input = fs::read_to_string("inputs/day21.txt").expect("Could not load input");

    let codes = input.lines().collect();

    println!("Part 1: {}", part1(&codes));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        let input = "029A
980A
179A
456A
379A"
            .lines();
        let res = part1(&input.collect());
        assert_eq!(126384, res);
    }

    #[test]
    fn test_p1_2() {
        let input = "208A
586A
341A
463A
593A"
            .lines();
        let res = part1(&input.collect());
        assert_eq!(126384, res);
    }
}
