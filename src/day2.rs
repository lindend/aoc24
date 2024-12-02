use std::fs;

fn parse_input_line(line: &str) -> Option<Vec<i32>> {
    if line.trim().is_empty() {
        None
    } else {
        Some(line
            .split_whitespace()
            .filter_map(|l| l.parse::<i32>().ok())
            .collect())
    }
}

fn is_correct_report(v: &[i32]) -> bool {
    let mut prev = v[0];
    let sign = i32::signum(v[1] - v[0]);
    for i in 1..v.len() {
        let diff = v[i] - prev;
        prev = v[i];
        if i32::abs(diff) > 3 || diff == 0 || i32::signum(diff) != sign {
            return false;
        }
    }
    true
}

fn is_correct_report_p2_v2(v: &[i32]) -> bool {
    for i in 0..v.len() {
        let mut vec = v.to_vec();
        vec.remove(i);
        if is_correct_report(&vec) {
            return true;
        }
    }

    false
}

fn part1(input: &Vec<Vec<i32>>) -> usize {
    input.iter()
        .filter(|v| is_correct_report(v))
        .count()
}

fn part2(input: &Vec<Vec<i32>>) -> usize {
    input.iter()
        .filter(|v| is_correct_report_p2_v2(v))
        .count()
}

pub fn day2() {
    let input = fs::read_to_string("inputs/day2.txt")
        .expect("Could not read input")
        .lines()
        .filter_map(parse_input_line)
        .collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_correct_1() {
        assert_eq!(true, is_correct_report(&vec!(1, 2, 3, 4, 5)));
    }

    #[test]
    fn test_is_correct_2() {
        assert_eq!(true, is_correct_report(&vec!(1, 2, 3, 4, 5)));
    }

    #[test]
    fn test_is_correct_3() {
        assert_eq!(true, is_correct_report(&vec!(1, 2, 3, 4, 7)));
    }

    #[test]
    fn test_wrong_1() {
        assert_eq!(false, is_correct_report(&vec!(1, 2, 1, 2, 3)));
    }

    #[test]
    fn test_wrong_2() {
        assert_eq!(false, is_correct_report(&vec!(1, 2, 3, 4, 8)));
    }

    #[test]
    fn test_wrong_3() {
        assert_eq!(false, is_correct_report(&vec!(1, 2, 2, 4, 5)));
    }

    // p2
    #[test]
    fn test_p2_is_correct_1() {
        assert_eq!(true, is_correct_report_p2_v2(&vec!(1, 2, 3, 4, 10)));
    }

    #[test]
    fn test_p2_is_correct_2() {
        assert_eq!(true, is_correct_report_p2_v2(&vec!(1, 2, 1, 3, 4, 5)));
    }

    #[test]
    fn test_p2_is_correct_3() {
        assert_eq!(true, is_correct_report_p2_v2(&vec!(20, 2, 3, 4, 5)));
    }

    #[test]
    fn test_p2_is_correct_4() {
        assert_eq!(true, is_correct_report_p2_v2(&vec!(8, 6, 4, 4, 1)));
    }

    #[test]
    fn test_p2_is_correct_10() {
        assert_eq!(true, is_correct_report_p2_v2(&vec!(8, 7, 4, 4, 1)));
    }

    #[test]
    fn test_p2_is_wrong_1() {
        assert_eq!(false, is_correct_report_p2_v2(&vec!(1, 2, 1, 3, 4, 10)));
    }

    #[test]
    fn test_p2_is_wrong_2() {
        assert_eq!(false, is_correct_report_p2_v2(&vec!(9, 7, 6, 2, 1)));
    }
}