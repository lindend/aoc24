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
    prev_inputs: &Vec<Input>,
) -> Option<Vec<Input>>
where
    F: Copy + Fn(&Vec<Input>) -> Vec<Input>,
{
    if targets.is_empty() {
        return Some(cmd_processor(&prev_inputs));
    }

    let target = targets[0];

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

    {
        let mut inputs = x_moves.clone();
        inputs.extend(y_moves.clone());
        inputs.push(Input::A);
        combinations.push(inputs);
    }

    if y_moves.len() > 1 {
        let mut inputs = Vec::new();
        inputs.extend(y_moves[..1].iter().cloned());
        inputs.extend(x_moves.clone());
        inputs.extend(y_moves[1..].iter().cloned());
        inputs.push(Input::A);
        combinations.push(inputs);
    }

    if x_moves.len() > 1 {
        let mut inputs = Vec::new();
        inputs.extend(x_moves[..1].iter().cloned());
        inputs.extend(y_moves.clone());
        inputs.extend(x_moves[1..].iter().cloned());
        inputs.push(Input::A);
        combinations.push(inputs);
    }

    let mut best_combination: Option<Vec<Input>> = None;

    for combination in &combinations {
        if is_bad(pos, &combination, bad) {
            continue;
        }

        let mut all_inputs = prev_inputs.clone();
        all_inputs.extend(combination.iter().cloned());
        let input = input_sequences(&targets[1..], target, bad, cmd_processor, &all_inputs);
        if let Some(input) = input {
            if let Some(ref best) = best_combination {
                if input.len() < best.len() {
                    best_combination = Some(input);
                }
            } else {
                best_combination = Some(input);
            }
        }
    }

    best_combination
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
        let inputs = input_sequences(
            &[target],
            curr_pos,
            Vec2::new(0, 3),
            dir_pad_0_sequences,
            &Vec::new(),
        );
        res.extend(inputs.unwrap());
        curr_pos = target;
    }
    res
}

fn hash(code: &Vec<Input>) -> u64 {
    let mut hasher = DefaultHasher::new();
    code.hash(&mut hasher);
    hasher.finish()
}

fn dir_pad_0_sequences(code: &Vec<Input>) -> Vec<Input> {
    {
        let memo = GLOBAL_MEMO.lock().unwrap();
        let key = (1, hash(code));
        if let Some(res) = memo.get(&key) {
            return res.clone();
        }
    }

    let mut hasher = DefaultHasher::new();
    code.hash(&mut hasher);
    let targets: Vec<_> = code
        .iter()
        .map(|c| match c {
            Input::Up => Vec2::new(1, 0),
            Input::A => Vec2::new(2, 0),
            Input::Left => Vec2::new(0, 1),
            Input::Down => Vec2::new(1, 1),
            Input::Right => Vec2::new(2, 1),
        })
        .collect();

    println!("Dir pad 0 processing code: {code:?}");
    let res = input_sequences(
        &targets,
        Vec2::new(2, 0),
        Vec2::new(0, 0),
        dir_pad_1_sequences,
        &Vec::new(),
    );

    {
        let mut memo = GLOBAL_MEMO.lock().unwrap();
        let key = (1, hash(code));
        memo.insert(key, res.clone().unwrap().clone());
    }

    res.unwrap()
}

fn dir_pad_1_sequences(code: &Vec<Input>) -> Vec<Input> {
    {
        let memo = GLOBAL_MEMO.lock().unwrap();
        let key = (2, hash(code));
        if let Some(res) = memo.get(&key) {
            return res.clone();
        }
    }

    let targets: Vec<_> = code
        .iter()
        .map(|c| match c {
            Input::Up => Vec2::new(1, 0),
            Input::A => Vec2::new(2, 0),
            Input::Left => Vec2::new(0, 1),
            Input::Down => Vec2::new(1, 1),
            Input::Right => Vec2::new(2, 1),
        })
        .collect();

    println!("Dir pad 1 processing code: {code:?}");
    let res = input_sequences(
        &targets,
        Vec2::new(2, 0),
        Vec2::new(0, 0),
        |input: &Vec<Input>| input.clone(),
        &Vec::new(),
    );
    {
        let mut memo = GLOBAL_MEMO.lock().unwrap();
        let key = (2, hash(code));
        memo.insert(key, res.clone().unwrap().clone());
    }
    res.unwrap()
}

fn part1(codes: &Vec<&str>) -> i64 {
    let mut total_complexity = 0;
    for &code in codes {
        let ks = keypad_sequences(code);

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
        println!("{}", res);
        assert_eq!(res, 155252);
    }
}

/*

029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
029A: <vA<AA>>^AvAA<^A>Av<<A>>^AvA^A<vA>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A

980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
980A: v<<A>>^AAAvA^A<vA<AA>>^AvAA<^A>Av<<A>A>^AAAvA<^A>A<vA>^A<A>A

179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
179A: v<<A>>^A<vA<A>>^AAvAA<^A>Av<<A>>^AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A

456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
456A: v<<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>Av<<A>A>^AAvA<^A>A

379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
379A: v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A


*/
