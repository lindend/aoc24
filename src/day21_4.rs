use crate::util::vec2::Vec2;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{LazyLock, Mutex};
use std::{fs, iter};

type Memo = HashMap<(i32, u64), Vec<Input>>;

static GLOBAL_MEMO: LazyLock<Mutex<Memo>> = LazyLock::new(|| Mutex::new(Memo::new()));

#[derive(Clone, PartialEq, Eq, Hash)]
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

impl Debug for Input {
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

fn is_bad(pos: Vec2<i32>, inputs: &Vec<Input>, bad: Vec2<i32>) -> bool {
    let mut curr_pos = pos;

    for input in inputs {
        let delta = match input {
            Input::Left => Vec2::new(-1, 0),
            Input::Right => Vec2::new(1, 0),
            Input::Up => Vec2::new(0, -1),
            Input::Down => Vec2::new(0, 1),
            Input::A => Vec2::zero(),
        };
        curr_pos = curr_pos + delta;

        if curr_pos == bad {
            return true;
        }
    }
    false
}

fn input_sequences<F>(
    targets: &[Vec2<i32>],
    pos: Vec2<i32>,
    bad: Vec2<i32>,
    cmd_processor: F,
) -> Option<Vec<Input>>
where
    F: Copy + Fn(&Vec<Input>) -> Vec<Input>,
{
    let mut pos = pos;
    let mut possible_sequences: Vec<Vec<Input>> = vec![Vec::new()];

    for &target in targets {
        let delta = target - pos;

        let x_moves = repeat(delta.x, Input::Right, Input::Left);
        let y_moves = repeat(delta.y, Input::Down, Input::Up);

        let mut combinations = Vec::new();

        {
            let mut inputs = y_moves.clone();
            inputs.extend(x_moves.clone());
            inputs.push(Input::A);
            combinations.push(inputs);
        }

        if delta.x != 0 && delta.y != 0 {
            let mut inputs = x_moves.clone();
            inputs.extend(y_moves.clone());
            inputs.push(Input::A);
            combinations.push(inputs);
        }

        let prev_inputs = possible_sequences;
        possible_sequences = Vec::new();

        for combination in &combinations {
            if is_bad(pos, &combination, bad) {
                continue;
            }

            let mut all_inputs = prev_inputs.clone();
            all_inputs
                .iter_mut()
                .for_each(|ai| ai.extend(combination.iter().cloned()));
            possible_sequences.extend(all_inputs);
        }
        pos = target;
    }

    possible_sequences
        .iter()
        .map(|s| cmd_processor(s))
        .min_by_key(|k| k.len())
}

fn keypad_sequences(code: &str, levels: i32) -> Vec<Input> {
    let targets: Vec<_> = code
        .chars()
        .map(|c| match c {
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
        })
        .collect();

    let mut res = Vec::new();

    let mut curr_pos = Vec2::new(2, 3);

    for target in targets {
        let inputs = input_sequences(&[target], curr_pos, Vec2::new(0, 3), |inputs| {
            dir_pad_sequences(inputs, levels - 1)
        });
        res.extend(inputs.unwrap());
        curr_pos = target;
    }
    res
}

fn hash(code: &[Input]) -> u64 {
    let mut hasher = DefaultHasher::new();
    code.hash(&mut hasher);
    hasher.finish()
}

fn dir_pad_sequences(code2: &Vec<Input>, level: i32) -> Vec<Input> {
    assert!(level >= 0);
    let splits = code2.split_inclusive(|c| *c == Input::A);

    let mut result = Vec::new();

    for split in splits {
        let cache_level = level;
        if level > 5 {
            println!("Dir pad level {level}, code: {code2:?}, split: {split:?}");
        }
        let cached = {
            let memo = GLOBAL_MEMO.lock().unwrap();
            let key = (cache_level, hash(split));
            memo.get(&key).map(|c| c.clone())
        };

        if let Some(res) = cached {
            if level > 5 {
                println!("Cache hit! level {level}");
            }
            result.extend(res.clone());
            continue;
        }

        let targets: Vec<_> = split
            .iter()
            .map(|c| match c {
                Input::Up => Vec2::new(1, 0),
                Input::A => Vec2::new(2, 0),
                Input::Left => Vec2::new(0, 1),
                Input::Down => Vec2::new(1, 1),
                Input::Right => Vec2::new(2, 1),
            })
            .collect();

        let res = input_sequences(&targets, Vec2::new(2, 0), Vec2::new(0, 0), |inputs| {
            if level == 0 {
                inputs.clone()
            } else {
                dir_pad_sequences(inputs, level - 1)
            }
        });

        {
            let mut memo = GLOBAL_MEMO.lock().unwrap();
            let key = (cache_level, hash(split));
            memo.insert(key, res.clone().unwrap().clone());
        }

        result.extend(res.unwrap());
    }
    result
}

fn enter_code(codes: &Vec<&str>, levels: i32) -> i64 {
    let mut total_complexity = 0;
    for &code in codes {
        let ks = keypad_sequences(code, levels);

        println!(
            "{code}: {}",
            ks.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        );

        let num_code: i64 = code[..code.len() - 1]
            .parse()
            .expect("Could not parse numeric part of code");
        let complexity = ks.len() as i64 * num_code;
        total_complexity += complexity;
    }
    // Too high: 157596

    // Wrong: 153888

    total_complexity
}

fn part1(codes: &Vec<&str>) -> i64 {
    enter_code(codes, 2)
}

fn part2(codes: &Vec<&str>) -> i64 {
    enter_code(codes, 25)
    // 1014360
}

pub fn day21() {
    let input = fs::read_to_string("inputs/day21.txt").expect("Could not load input");

    let codes = input.lines().collect();

    println!("Part 1: {}", part1(&codes));
    println!("Part 2: {}", part2(&codes));
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
        println!("{}", res);
        assert_eq!(res, 155252);
    }
}

/*

029A: v<A<AA>>^AvAA^<A>Av<<A>>^AvA^Av<A^>A<Av<A>>^AAAvA^AvA^Av<A<A>>^AAAAA<Av>A^A
980A: v<<A>>^Av<<A>>^AAAAAvA^AvA^Av<A<AA>>^AvAA^<A>Av<A<AA>>^Av<A<AA>>^AvAA^<A>AvAA^<A>Av<A<A>>^AAAAA<Av>A^Av<A<A>>^Av<A<A>>^AAAAA<Av>A^A<Av>A^Av<A^>Av<A^>A<A>A<A>A
179A: v<<A>>^Av<<A>>^Av<A<A>>^Av<A<A>>^AAAvAA^<A>AvAA^<A>Av<<A>>^Av<<A>>^AAAvA^AvA^Av<A^>Av<A^>AAA<A>A<A>Av<A<A>>^Av<A<A>>^AAAAA<Av>A^A<Av>A^Av<A<A>>^Av<A<A>>^AAAAA<Av>A^A<Av>A^A
456A: v<<A>>^Av<<A>>^AAAv<A<A>>^Av<A<A>>^AAAvAA^<A>AvAA^<A>Av<A^>Av<A^>A<A>A<A>Av<A^>Av<A^>A<A>A<A>Av<A^>Av<A^>A<A>A<A>Av<A^>Av<A^>A<A>A<A>Av<A<A>>^Av<A<A>>^AAA<Av>A^A<Av>A^A
379A: v<<A>>^AvA^Av<<A>>^Av<<A>>^AvA^AvA^Av<A<AA>>^Av<A<AA>>^AAAvA^<A>AvA^<A>AAAvA^AvA^Av<A^>Av<A^>AAA<A>A<A>Av<A^>Av<A^>AAA<A>A<A>Av<A<A>>^Av<A<A>>^AAAAA<Av>A^A<Av>A^Av<A<A>>^Av<A<A>>^AAAAA<Av>A^A<Av>A^A


REAL:

029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

COMBINED:
029A: v<A<AA>>^AvAA^<A>Av<<A>>^AvA^Av<A^>A <Av<A>>^AAAvA^AvA^Av<A<A>>^AAAAA<Av>A^A
029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A <v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A




*/
