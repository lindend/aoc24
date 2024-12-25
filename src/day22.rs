use std::{
    collections::{HashMap, HashSet},
    fs,
};

/*

secret * 64
mix into secret
prune

secret / 32
round
mix -> secret
prune

secret * 2048
mix -> secret
prune


----
mix:
value XOR secret

prune:
value mod 16777216

*/

fn mix(n1: i64, n2: i64) -> i64 {
    n1 ^ n2
}

fn prune(n: i64) -> i64 {
    n % 16777216
}

fn next_secret(secret: i64) -> i64 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    let secret = prune(mix(secret, secret * 2048));
    secret
}

fn ones(n: i64) -> i64 {
    n % 10
}

fn part1(secrets: &[i64]) -> i64 {
    secrets
        .iter()
        .map(|&s| {
            let mut s = s;
            for i in 0..2000 {
                s = next_secret(s);
            }
            s as i64
        })
        .sum()
}

fn part2(secrets: &[i64]) -> i64 {
    let mut best_sequences = HashMap::new();
    for &secret in secrets {
        let mut sequences = HashSet::new();
        let mut current = secret;
        let mut d0 = None;
        let mut d1 = None;
        let mut d2 = None;
        let mut d3 = None;
        for _ in 0..2000 {
            let next = next_secret(current);
            let prev_ones = ones(current);
            let current_ones = ones(next);
            let delta = current_ones - prev_ones;
            current = next;
            d0 = d1;
            d1 = d2;
            d2 = d3;
            d3 = Some(delta);
            if d0.is_some() {
                let key = (d0.unwrap(), d1.unwrap(), d2.unwrap(), d3.unwrap());
                if !sequences.insert(key) {
                    continue;
                }

                best_sequences
                    .entry(key)
                    .and_modify(|e: &mut Vec<i64>| e.push(current_ones))
                    .or_insert(vec![current_ones]);
            }
        }
    }

    best_sequences
        .iter()
        .map(|(_, v)| v.iter().sum())
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .parse()
                .expect(format!("Could not parse `{l}` as int").as_str())
        })
        .collect()
}

pub fn day22() {
    let input = fs::read_to_string("inputs/day22.txt").expect("Could not read input file");
    let secrets = parse_input(&input);
    println!("Part 1: {}", part1(&secrets));
    println!("Part 2: {}", part2(&secrets));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "1
    10
    100
    2024";
        let secrets = parse_input(&input);
        assert_eq!(37327623, part1(&secrets));
    }

    #[test]
    fn test_p2() {
        let input = "1
2
3
2024";
        let secrets = parse_input(&input);
        assert_eq!(23, part2(&secrets));
    }
}
