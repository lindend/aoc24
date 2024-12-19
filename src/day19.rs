use std::collections::HashMap;
use std::fs;
fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, patterns) = input.split_once("\n\n").expect("Invalid input");

    let towels: Vec<_> = towels.split(",")
        .map(|t| t.trim())
        .filter(|t| t.len() > 0)
        .collect();

    let patterns = patterns.lines().collect();

    (towels, patterns)
}

fn match_pattern<'a>(towels: &'a [&str], pattern: &'a str, memoize: &mut HashMap<&'a str, i64>) -> i64 {
    if pattern.len() == 0 {
        return 1;
    }

    towels.iter()
        .filter(|&&t| pattern.starts_with(t))
        .map(|&t| {
            let p = &pattern[t.len()..];
            if memoize.contains_key(p) {
                *memoize.get(p).unwrap()
            } else {
                let res = match_pattern(towels, p, memoize);
                memoize.insert(p, res);
                res
            }
        })
        .sum()
}

fn part1(towels: &Vec<&str>, patterns: &Vec<&str>) -> i32 {
    patterns.iter()
        .filter(|&p| match_pattern(towels, p, &mut HashMap::new()) > 0)
        .count() as i32
}

fn part2(towels: &Vec<&str>, patterns: &Vec<&str>) -> i64 {
    patterns.iter()
        .map(|&p| match_pattern(towels, p, &mut HashMap::new()))
        .sum()
}

pub fn day19() {
    let input = fs::read_to_string("inputs/day19.txt").expect("Could not read input");

    let (towels, patterns) = parse_input(&input);

    println!("Part 1: {}", part1(&towels, &patterns));
    println!("Part 2: {}", part2(&towels, &patterns));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let (towels, patterns) = parse_input(&input);
        assert_eq!(6, part1(&towels, &patterns));
    }

    #[test]
    fn test_p2() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let (towels, patterns) = parse_input(&input);
        assert_eq!(16, part2(&towels, &patterns));
    }
}