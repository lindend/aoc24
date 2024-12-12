use std::fs;

fn check_pattern(input: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32, search: &Vec<char>) -> bool {
    let max_x = input.len() as i32;
    let max_y = input[0].len() as i32;

    for i in 1..search.len() {
        let px = x as i32 + dx * i as i32;
        let py = y as i32 + dy * i as i32;

        if px < 0 || px >= max_x || py < 0 || py >= max_y {
            return false;
        }

        if input[px as usize][py as usize] != search[i] {
            return false;
        }
    }

    true
}

pub fn part1_v2(input: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let search: Vec<_> = "XMAS".chars().collect();
    let patterns = [(1i32, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];

    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if input[x][y] == search[0] {
                for (dx, dy) in patterns {
                    if check_pattern(&input, x, y, dx, dy, &search) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn is_m_and_s(input: &Vec<Vec<char>>, x: usize, y: usize, (dx, dy): (i32, i32)) -> bool {
    let x = x as i32;
    let y = y as i32;
    let px = (x + dx) as usize;
    let py = (y + dy) as usize;
    let nx = (x - dx) as usize;
    let ny = (y - dy) as usize;

    (input[px][py] == 'M' && input[nx][ny] == 'S') ||
        (input[px][py] == 'S' && input[nx][ny] == 'M')
}

pub fn part2(input: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let patterns = [(1i32, 1), (1, -1)];

    for x in 1..input.len() - 1 {
        for y in 1..input[x].len() - 1 {
            if input[x][y] == 'A' {
                if is_m_and_s(&input, x, y, patterns[0]) && is_m_and_s(&input, x, y, patterns[1]) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn day4() {
    let input: Vec<Vec<char>> = fs::read_to_string("inputs/day4.txt")
        .expect("Could not read input")
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();

    println!("Part 1: {}", part1_v2(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_works() {
        assert_eq!(18, part1_v2(
            &"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".lines().map(|l| l.chars().collect()).collect()))
    }

    #[test]
    pub fn test_p2_works() {
        assert_eq!(9, part2(
            &"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX".lines().map(|l| l.chars().collect()).collect()))
    }
}