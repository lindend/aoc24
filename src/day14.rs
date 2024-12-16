use crate::util::vec2::Vec2;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

type Robot = (Vec2<i32>, Vec2<i32>);

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=([0-9-]+),([0-9-]++) v=([0-9-]+),([0-9-]+)").expect("invalid regex");
    input.lines()
        .filter_map(|l| re.captures(l))
        .map(|m| m.extract())
        .map(|(_, [px, py, vx, vy])| (Vec2::new(px.parse().unwrap(), py.parse().unwrap()), Vec2::new(vx.parse().unwrap(), vy.parse().unwrap())))
        .collect()
}

fn move_robots(robots: &[Robot], room_size: Vec2<i32>, num_steps: i32) -> Vec<Vec2<i32>> {
    robots.iter()
        .map(|(p, v)| *p + *v * num_steps)
        .map(|p| Vec2::new(p.x.rem_euclid(room_size.x), p.y.rem_euclid(room_size.y)))
        .collect()
}

fn is_christmas_tree(ps: &Vec<Vec2<i32>>, room_size: Vec2<i32>) -> i32 {
    let half_room = room_size / 2;
    // let hs: HashSet<Vec2<i32>> = HashSet::from_iter(ps.iter().cloned());
    // let mut num = 0;
    // for y in 0..room_size.y {
    //     let tree_pos = Vec2::new(half_room.x, y);
    //     if hs.contains(&tree_pos) {
    //         num += 1;
    //     }
    // }
    // num

    let mut num_at_positions: HashMap<Vec2<i32>, i32> = HashMap::new();
    for &p in ps {
        num_at_positions.entry(p).and_modify(|mut n| *n += 1).or_insert(1);
    }

    let mut max_nc = 0;
    for y in 0..room_size.y {
        let mut nc = 0;
        for x in 0..room_size.x {
            if num_at_positions.contains_key(&Vec2::new(x, y)) {
                nc += 1;
                if nc > max_nc {
                    max_nc = nc;
                }
            } else {
                nc = 0;
            }
        }
    }

    max_nc
}

fn print_robots(rs: &Vec<Vec2<i32>>, room_size: Vec2<i32>) -> String {
    let mut res = String::new();
    res.reserve(((room_size.x + 2) * room_size.y) as usize);
    let mut num_at_positions: HashMap<Vec2<i32>, u8> = HashMap::new();
    for &p in rs {
        num_at_positions.entry(p).and_modify(|mut n| *n += 1).or_insert(1);
    }

    for y in 0..room_size.y {
        for x in 0..room_size.x {
            res.push(
                num_at_positions
                    .get(&Vec2::new(x as i32, y as i32))
                    .map(|&n| ((n + ('0' as u8)) as char))
                    .unwrap_or('.'))
        }
        res.push('\n');
    }

    res
}

fn part1(robots: &[Robot], room_size: Vec2<i32>) -> i32 {
    let new_positions = move_robots(&robots, room_size, 100);
    let half_size = room_size / 2;
    let quadrants = vec![
        Vec2::zero(), Vec2::new(half_size.x + 1, 0),
        Vec2::new(0, half_size.y + 1), half_size + Vec2::one()
    ];

    quadrants
        .iter()
        .map(|&q|
            new_positions.iter()
                .filter(|&p| p.in_bounds(q, q + half_size - Vec2::one()))
                .count()
                .max(1) as i32
        )
        .fold(1, |a, n| a * n)
}

fn part2(robots: &[Robot], room_size: Vec2<i32>) -> i32 {
    let mut max_n = 0;
    let mut max_n_i = 0;
    for i in 1..10000 {
        let ps = move_robots(&robots, room_size, i);
        let nm = is_christmas_tree(&ps, room_size);
        if nm > max_n {
            max_n = nm;
            max_n_i = i;
        }
        if nm > 7 {
            println!("{i}:\n{}", &print_robots(&ps, room_size));
            println!();
            println!();
            println!();
        }

        if i % 2000 == 0 {
            println!("{i}, max n: {max_n}, max n i: {max_n_i}");
        }
    }
    0
}

pub fn day14() {
    let input = fs::read_to_string("inputs/day14.txt").expect("Could not read input");
    let robots = parse_input(&input);

    println!("Part 1: {}", part1(&robots, Vec2::new(101, 103)));
    println!("Part 2: {}", part2(&robots, Vec2::new(101, 103)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let robots = parse_input(&input);
        let room_size = Vec2::new(11, 7);
        assert_eq!(12, part1(&robots, room_size));
    }
}