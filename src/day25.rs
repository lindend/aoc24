use std::fs;

use crate::util::str_util::transpose;

type Schematic = [i32; 5];

fn parse_input(input: &str) -> (Vec<Schematic>, Vec<Schematic>) {
    let mut keys: Vec<Schematic> = Vec::new();
    let mut locks: Vec<Schematic> = Vec::new();

    for schematic in input.split("\n\n") {
        let schematic = schematic.trim();
        let is_key = schematic.starts_with("#");

        let heights: Schematic = transpose(schematic)
            .iter()
            .map(|s| s.chars().filter(|&c| c == '#').count() as i32 - 1)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        if is_key {
            keys.push(heights);
        } else {
            locks.push(heights);
        }
    }
    (keys, locks)
}

fn part1(keys: &Vec<Schematic>, locks: &Vec<Schematic>) -> i32 {
    let mut num_combinations = 0;

    for k in keys {
        'locks: for l in locks {
            for pin in 0..5 {
                if k[pin] + l[pin] > 5 {
                    continue 'locks;
                }
            }
            num_combinations += 1;
        }
    }

    num_combinations
}

pub fn day25() {
    let input = fs::read_to_string("inputs/day25.txt").expect("Could not read input");
    let (keys, locks) = parse_input(&input);

    println!("Part 1: {}", part1(&keys, &locks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";
        let (keys, locks) = parse_input(&input);
        println!("{keys:?}");
        println!("{locks:?}");
        assert_eq!(3, part1(&keys, &locks));
    }
}
