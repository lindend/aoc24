use crate::util::vec2::Vec2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::fs;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Input {
    Left,
    Right,
    Up,
    Down,
    A,
}

impl Input {
    fn from_vec(v: Vec2<i32>) -> Self {
        match v {
            Vec2 { x: -1, y: 0 } => Input::Left,
            Vec2 { x: 1, y: 0 } => Input::Right,
            Vec2 { x: 0, y: -1 } => Input::Up,
            Vec2 { x: 0, y: 1 } => Input::Down,
            Vec2 { x: 0, y: 0 } => Input::A,
            _ => panic!("Invalid delta"),
        }
    }
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

#[derive(PartialEq, Eq, Debug)]
struct State<'a, T: Clone + std::fmt::Debug> {
    cost: i32,
    heuristic: i32,
    pos: Vec2<i32>,
    prev_pos: Vec2<i32>,
    targets: &'a [Vec2<i32>],
    path: Vec<Vec2<i32>>,
    cost_state: T,
}

impl<'a, T: Eq + PartialEq + Clone + Debug> Ord for State<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl<'a, T: Eq + PartialEq + Clone + Debug> PartialOrd for State<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Copy, Debug)]
struct CostState {
    keypad_pos: Vec2<i32>,
    dirpad_0_pos: Vec2<i32>,
    dirpad_1_pos: Vec2<i32>,
}

fn shortest_path<T, ST: Eq + PartialEq + Clone + Debug>(
    start: Vec2<i32>,
    targets: &Vec<Vec2<i32>>,
    obstacles: &HashSet<Vec2<i32>>,
    cost_fn: fn(Vec2<i32>, Vec2<i32>, &ST, &mut T) -> (i32, ST),
    cost_context: &mut T,
    cost_state: ST,
) -> (i32, Vec<Vec2<i32>>, ST) {
    let mut heads = BinaryHeap::new();
    heads.push(State {
        cost: 0,
        heuristic: (start - targets[0]).manhattan_distance(),
        pos: start,
        prev_pos: start,
        targets: targets.as_slice(),
        path: Vec::new(),
        cost_state,
    });

    let mut visited = HashMap::new();

    let all_dirs = Vec2::all_dirs();

    loop {
        let head = heads.pop().expect("No path");

        visited.insert(head.pos, head.cost);

        let mut targets = head.targets;

        assert!(!targets.is_empty());

        let mut cost = head.cost;
        let mut path = head.path.clone();

        if head.pos == targets[0] {
            cost += 1;
            path.push(Vec2::zero());
            targets = &targets[1..];
        }

        if targets.is_empty() {
            return (head.cost, path, head.cost_state);
        }

        for &dir in &all_dirs {
            let pos = head.pos + dir;
            if pos == head.prev_pos {
                continue;
            }

            let (c, cost_state) = cost_fn(head.pos, pos, &head.cost_state, cost_context);
            let cost = cost + c;

            if obstacles.contains(&pos) {
                continue;
            }
            let mut path = path.clone();

            path.push(dir);

            heads.push(State {
                cost,
                heuristic: cost + (pos - targets[0]).manhattan_distance(),
                pos,
                prev_pos: head.pos,
                targets,
                path,
                cost_state,
            });
        }
    }
}

fn keypad_sequences(
    code: &str,
    memo: &mut HashMap<(CostState, PadId, Vec2<i32>), (i32, CostState)>,
) -> (i32, Vec<Vec2<i32>>) {
    let targets = code
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
    let mut obstacles = HashSet::new();
    obstacles.insert(Vec2::new(0, 3));
    let (cost, path, _) = shortest_path(
        Vec2::new(2, 3),
        &targets,
        &obstacles,
        |from, to, &state, context| {
            let (cost, mut state) = dir_pad_sequences(&state, to - from, PadId::Dirpad0, context);
            println!("Navigated a key");
            state.keypad_pos = to;
            (cost + 1, state)
        },
        memo,
        CostState {
            keypad_pos: Vec2::new(2, 3),
            dirpad_0_pos: Vec2::new(2, 0),
            dirpad_1_pos: Vec2::new(2, 0),
        },
    );
    (cost, path)
}

#[derive(PartialEq, Eq, Clone, Hash)]
enum PadId {
    Numpad,
    Dirpad0,
    Dirpad1,
}

fn dir_pad_sequences(
    state: &CostState,
    delta: Vec2<i32>,
    id: PadId,
    memo: &mut HashMap<(CostState, PadId, Vec2<i32>), (i32, CostState)>,
) -> (i32, CostState) {
    let input = Input::from_vec(delta);
    let target = match input {
        Input::Up => Vec2::new(1, 0),
        Input::A => Vec2::new(2, 0),
        Input::Left => Vec2::new(0, 1),
        Input::Down => Vec2::new(1, 1),
        Input::Right => Vec2::new(2, 1),
    };

    if let Some((cost, new_state)) = memo.get(&(*state, id.clone(), delta)) {
        return (*cost, new_state.clone());
    }

    let mut obstacles = HashSet::new();
    obstacles.insert(Vec2::zero());

    let start = if id == PadId::Dirpad0 {
        println!("Navigating {input}");
        state.dirpad_0_pos
    } else {
        state.dirpad_1_pos
    };

    let (c, _, new_state) = shortest_path(
        start,
        &vec![target],
        &obstacles,
        if id == PadId::Dirpad0 {
            |from: Vec2<i32>, to, state, context| {
                let (c, mut state) = dir_pad_sequences(state, to - from, PadId::Dirpad1, context);
                state.dirpad_0_pos = to;
                (c + 1, state)
            }
        } else {
            |_, to, &state, _| {
                (
                    1,
                    CostState {
                        keypad_pos: state.keypad_pos,
                        dirpad_0_pos: state.dirpad_0_pos,
                        dirpad_1_pos: to,
                    },
                )
            }
        },
        memo,
        *state,
    );
    memo.insert((*state, id, delta), (c, new_state));

    (c, new_state.clone())
}

fn part1(codes: &Vec<&str>) -> i64 {
    let mut total_complexity = 0;
    let mut memo = HashMap::new();
    for &code in codes {
        let (code_len, path) = keypad_sequences(code, &mut memo);
        let path: Vec<_> = path
            .iter()
            .map(|&p| Input::from_vec(p).to_string())
            .collect();
        let path = path.join("");

        println!("Found sequence {code} len {code_len} {path}");

        let num_code: i64 = code[..code.len() - 1]
            .parse()
            .expect("Could not parse numeric part of code");
        let complexity = code_len as i64 * num_code;
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
593A
        "
        .lines();
        let res = part1(&input.collect());
        assert_eq!(126384, res);
    }
}
