use regex::Regex;
use std::fs;

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [num1, num2])| (num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap()))
        .map(|(n1, n2)| n1 * n2)
        .sum()
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"((mul)\((\d{1,3}),(\d{1,3})\))|((do)\(\)()())|((don't)\(\))()()").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [_, cmd, p1, p2])| [cmd, p1, p2])
        .fold((true, 0), |(e, sum), cmd| match (e, cmd) {
            (true, ["mul", n1, n2]) => (e, sum + n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap()),
            (_, ["do", _, _]) => (true, sum),
            (_, ["don't", _, _]) => (false, sum),
            _ => (e, sum)
        }).1
}

pub fn day3() {
    let input = fs::read_to_string("inputs/day3.txt").expect("Could not read input");

    println!("Part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_works() {
        assert_eq!(161, part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"));
    }

    #[test]
    fn test_p2_works() {
        assert_eq!(48, part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"));
    }
}