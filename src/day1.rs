use std::fs;

pub fn parse_input_line(line: &str) -> Option<(i32, i32)> {
    let line = line.trim();
    if line.is_empty() {
        None
    } else {
        line.split_once(" ")
            .map(|(l, r)| (l.trim().parse().expect(l), r.trim().parse().expect(r)))
    }
}

pub fn part1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut diff_sum = 0;

    for i in 0..left.len() {
        diff_sum += i32::abs(left[i] - right[i]);
    }

    diff_sum
}

pub fn part2(left: &Vec<i32>, right: &Vec<i32>) -> i64 {
    let mut score: i64 = 0;

    let mut ridx = 0;
    for &n in left {
        let mut count: i64 = 0;
        while ridx < right.len() && right[ridx] <= n {
            if right[ridx] == n {
                count += 1
            }
            ridx += 1;
        }
        score += i64::from(n) * count;
    }
    score
}

pub fn day1() {
    let (mut left, mut right): (Vec<_>, Vec<_>) = fs::read_to_string("inputs/day1.txt")
        .expect("Could not read input")
        .lines()
        .filter_map(parse_input_line)
        .unzip();

    left.sort();
    right.sort();

    println!("Diff is {}", part1(&left, &right));
    println!("Similarity score: {}", part2(&left, &right));
}
