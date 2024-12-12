use std::fs;

fn concat_numbers(num1: i64, num2: i64) -> i64 {
    (num1.to_string() + &*num2.to_string()).parse().expect("Could not concat numbers")
}

fn test_operations(target: &i64, current_sum: i64, nums: &[i64], ext: bool) -> bool {
    if current_sum > *target {
        false
    } else if nums.len() == 0 {
        *target == current_sum
    } else {
        test_operations(target, current_sum + nums[0], &nums[1..], ext) ||
            test_operations(target, current_sum * nums[0], &nums[1..], ext) ||
            (ext && test_operations(target, concat_numbers(current_sum, nums[0]), &nums[1..], ext))
    }
}

fn part1(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter()
        .filter(|(sum, operands)| test_operations(sum, operands[0], &operands[1..], false))
        .map(|(sum, _)| sum)
        .sum()
}

fn part2(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    input.iter()
        .filter(|(sum, operands)| test_operations(sum, operands[0], &operands[1..], true))
        .map(|(sum, _)| sum)
        .sum()
}


pub fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input.lines()
        .map(|l| l.split_once(":").unwrap())
        .map(|(sum, operands)| (
            sum.parse().expect("Could not parse sum"),
            operands.split_whitespace().filter_map(|o| o.parse().ok()).collect()
        ))
        .collect()
}

pub fn day7() {
    let input = fs::read_to_string("inputs/day7.txt").expect("Could not load input");
    let input = parse_input(&input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_p1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let p = parse_input(&input);
        assert_eq!(3749, part1(&p));
    }

    #[test]
    pub fn test_p2() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let p = parse_input(&input);
        assert_eq!(11387, part2(&p));
    }
}