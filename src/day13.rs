use crate::util::vec2::Vec2;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

pub struct Machine {
    button_a: Vec2<i64>,
    button_b: Vec2<i64>,
    prize: Vec2<i64>,
}

fn input_to_vec2(input: &str) -> Vec2<i64> {
    let input_regex = Regex::new(r"X.(\d+), Y.(\d+)").expect("Bad regex");
    let (_, [x, y]) = input_regex.captures(&input).expect("Could not match regex").extract();
    Vec2::new(x.parse().unwrap(), y.parse().unwrap())
}

pub fn parse_input(input: &str) -> Vec<Machine> {
    let mut res = Vec::new();
    let mut lines = input.lines();

    loop {
        let a = lines.next();
        if !a.is_some() {
            return res;
        }
        let button_a = input_to_vec2(a.unwrap());
        let button_b = input_to_vec2(lines.next().unwrap());
        let prize = input_to_vec2(lines.next().unwrap());
        lines.next();
        res.push(Machine {
            button_a,
            button_b,
            prize,
        });
    }
}

const A_COST: i64 = 3;
const B_COST: i64 = 1;

fn solve_machine(machine: &Machine) -> Option<i64> {
    let mut heads = vec![(machine.prize, 0, 0)];
    let mut cheapest = None;
    let max = Vec2::new(i64::MAX, i64::MAX);
    let mut visited = HashSet::new();

    while !heads.is_empty() {
        let (prize, num_a, num_b) = heads.pop().unwrap();
        let cost = num_a * A_COST + num_b * B_COST;
        if !visited.insert(prize) {
            continue;
        }
        if !prize.in_bounds(Vec2::zero(), max) || cost > cheapest.unwrap_or(i64::MAX) {
            continue;
        }

        if prize == Vec2::zero() {
            cheapest = Some(cost);
            continue;
        }

        let a = (prize - machine.button_a, num_a + 1, num_b);
        let b = (prize - machine.button_b, num_a, num_b + 1);

        heads.push(a);
        heads.push(b);
    }
    cheapest
}

fn get_primes(max: i64) -> Vec<i64> {
    let mut primes = vec![2];

    for i in 3..max {
        if !primes.iter().any(|p| i % p == 0) {
            primes.push(i);
        }
    }

    primes
}

fn get_prime_factors(num: i64, primes: &[i64]) -> Vec<i64> {
    let mut factors = Vec::new();

    for &p in primes {
        let mut pn = num;
        while pn % p == 0 {
            factors.push(p);
            pn = pn / p;
        }
    }

    factors
}

fn to_vec_float64(v: &Vec2<i64>) -> Vec2<f64> {
    Vec2::new(v.x as f64, v.y as f64)
}

fn solve_machine_v2(machine: &Machine) -> Option<i64> {
    let p = to_vec_float64(&machine.prize);
    let a = to_vec_float64(&machine.button_a);
    let b = to_vec_float64(&machine.button_b);
    let B = (a.y * p.x - a.x * p.y) / (a.y * b.x - a.x * b.y);
    let A = (p.y - B * b.y) / a.y;

    let ib = B as i64;
    let ia = A as i64;

    if machine.button_a * ia + machine.button_b * ib == machine.prize {
        Some(ia * A_COST + ib * B_COST)
    } else {
        None
    }
}

/*
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400



A ax + B bx = Px
A ay + B by = Py
A = (Py - B by) / ay

ax (Py - B by) / ay + B bx = Px
B bx = Px - ax Py / ay + ax B by / ay
ay B bx = ay Px - ax Py + ax B by
ay B bx - ax B by = ay Px - ax Py
B (ay bx - ax by) = (ay Px - ax Py)
B = (ay Px - ax Py) / (ay bx - ax by)

 */

pub fn part1(machines: &[Machine]) -> i64 {
    machines.iter()
        .filter_map(|m| solve_machine(m))
        .sum()
}


pub fn part2(machines: &[Machine], offset: i64) -> i64 {
    machines.iter()
        .map(|m| Machine {
            button_a: m.button_a,
            button_b: m.button_b,
            prize: m.prize + Vec2::new(offset, offset),
        })
        .filter_map(|m| solve_machine_v2(&m))
        .sum()
}
pub fn day13() {
    let input = fs::read_to_string("inputs/day13.txt").expect("Could not read input");
    let machines = parse_input(&input);
    println!("Part 1: {}", part1(&machines));
    println!("Part 2: {}", part2(&machines, 10000000000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let machines = parse_input(&input);
        assert_eq!(480, part1(&machines));
    }

    #[test]
    pub fn test_p2_1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let machines = parse_input(&input);
        assert_eq!(480, part2(&machines, 0));
    }

    #[test]
    pub fn test_p2_2() {
        let input = fs::read_to_string("inputs/day13.txt").unwrap();
        let machines = parse_input(&input);
        assert_eq!(36954, part2(&machines, 0));
    }
}