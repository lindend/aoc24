use crate::util::vec2::Vec2;
use std::collections::HashSet;
use std::fs;

pub struct Node {
    level: u32,
    pos: Vec2<i32>,
    children: Vec<Node>,
}


pub fn get_node(map: &Vec<Vec<u32>>, level: u32, pos: Vec2<i32>) -> Node {
    let mut children = Vec::new();
    if level < 9 {
        let sides = vec![Vec2::new(1, 0), Vec2::new(-1, 0), Vec2::new(0, 1), Vec2::new(0, -1)];
        let size = Vec2::new(map[0].len() as i32, map.len() as i32);
        for side in sides {
            let c = pos + side;
            if c.in_bounds(Vec2::zero(), size - Vec2::one()) {
                if map[c.y as usize][c.x as usize] == level + 1 {
                    children.push(get_node(&map, level + 1, c));
                }
            }
        }
    }

    Node {
        level,
        pos,
        children,
    }
}

pub fn count_peaks(trail: &Node, visited: &mut HashSet<Vec2<i32>>) -> i32 {
    if trail.level == 9 {
        if visited.insert(trail.pos) { 1 } else { 0 }
    } else {
        trail.children.iter().map(|node| count_peaks(node, &mut *visited)).sum()
    }
}

pub fn count_rating(trail: &Node) -> i32 {
    if trail.level == 9 {
        1
    } else {
        trail.children.iter().map(|node| crate::day10::count_rating(node)).sum()
    }
}

pub fn get_tree(map: &Vec<Vec<u32>>) -> Vec<Node> {
    let mut tree = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                let node = get_node(&map, 0, Vec2::new(x as i32, y as i32));
                tree.push(node);
            }
        }
    }
    tree
}

pub fn part1(input: &Vec<Vec<u32>>) -> i32 {
    let tree = get_tree(&input);

    let mut num_peaks = 0;
    for trail_head in tree {
        let mut peaks = HashSet::new();
        num_peaks += count_peaks(&trail_head, &mut peaks);
    }
    num_peaks
}

pub fn part2(input: &Vec<Vec<u32>>) -> i32 {
    let tree = get_tree(&input);

    tree.iter()
        .map(|n| count_rating(&n))
        .sum()
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).expect("Could not parse number"))
            .collect())
        .collect()
}
pub fn day10() {
    let input = fs::read_to_string("inputs/day10.txt")
        .expect("Could not read input");
    let input = parse_input(&input);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let map = parse_input(&input);
        assert_eq!(36, part1(&map));
    }

    #[test]
    pub fn test_p2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let map = parse_input(&input);
        assert_eq!(81, part2(&map));
    }
}