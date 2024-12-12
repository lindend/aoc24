use std::collections::HashMap;
use std::fs;

fn is_valid_update(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let page_pos: HashMap<i32, usize> = HashMap::from_iter(
        update.iter()
            .enumerate()
            .map(|(i, &u)| (u, i))
    );

    for (first, second) in rules {
        let first_pos = page_pos.get(first).copied().unwrap_or(0);
        let last_pos = page_pos.get(second).copied().unwrap_or(update.len());
        if first_pos > last_pos {
            return false;
        }
    }
    true
}

fn middle_sum(v: &Vec<&Vec<i32>>) -> i32 {
    v.iter().map(|vu| vu[vu.len() / 2]).sum()
}

fn part1(rules: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut valid_updates = Vec::new();

    for update in updates {
        if is_valid_update(update, &rules) {
            valid_updates.push(update);
        }
    }
    middle_sum(&valid_updates)
}

fn fix_update(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> Vec<i32> {
    let mut res = update.to_vec();

    let mut has_breaks = true;

    while has_breaks {
        has_breaks = false;
        for (first, second) in rules {
            let first_pos = res.iter().position(|p| p == first).unwrap_or(0);
            let second_pos = res.iter().position(|p| p == second).unwrap_or(update.len());
            if first_pos > second_pos {
                res.swap(first_pos, second_pos);
                has_breaks = true;
            }
        }
    }

    res
}

fn part2(rules: &Vec<(i32, i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        if !is_valid_update(update, &rules) {
            invalid_updates.push(fix_update(rules, &update));
        }
    }

    middle_sum(&invalid_updates.iter().map(|v| v).collect())
}

pub fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let input: Vec<_> = input
        .lines()
        .collect();

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for l in input {
        if l.contains("|") {
            let parts = l.split_once('|').expect("Invalid rule line");
            rules.push((parts.0.parse().unwrap(), parts.1.parse().unwrap()));
        } else if l.contains(",") {
            updates.push(
                l.split(",")
                    .map(|l| l.parse().unwrap())
                    .collect()
            )
        }
    }

    (rules, updates)
}

pub fn day5() {
    let input = fs::read_to_string("inputs/day5.txt").expect("Could not read input");
    let (rules, updates) = parse_input(&input);

    println!("Part 1 {}", part1(&rules, &updates));
    println!("Part 2 {}", part2(&rules, &updates));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn tes_p1() {
        let (rules, updates) = parse_input("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
        assert_eq!(143, part1(&rules, &updates));
    }

    #[test]
    pub fn tes_p2() {
        let (rules, updates) = parse_input("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
        assert_eq!(123, part2(&rules, &updates));
    }
}