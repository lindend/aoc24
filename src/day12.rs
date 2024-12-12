use crate::util::vec2::Vec2;
use std::collections::HashSet;
use std::fs;

pub struct Area {
    cells: HashSet<Vec2<i32>>,
}


pub fn find_cells(input: &Vec<Vec<char>>, pos: Vec2<i32>, name: char) -> HashSet<Vec2<i32>> {
    let mut cells = HashSet::new();
    cells.insert(pos);

    let all_dirs = vec![Vec2::new(1, 0), Vec2::new(-1, 0), Vec2::new(0, 1), Vec2::new(0, -1)];
    let mut heads: Vec<_> = all_dirs.iter().map(|&d| d + pos).collect();

    let size = Vec2::new(input[0].len() as i32 - 1, input.len() as i32 - 1);
    while !heads.is_empty() {
        let head = heads.pop().unwrap();
        if !head.in_bounds(Vec2::zero(), size) ||
            cells.contains(&head) ||
            input[head.y as usize][head.x as usize] != name {
            continue;
        }
        cells.insert(head);
        all_dirs
            .iter()
            .map(|&d| d + head)
            .for_each(|v| heads.push(v));
    }

    cells
}
pub fn parse_input(input: &str) -> Vec<Area> {
    let mut visited = HashSet::new();
    let input: Vec<Vec<_>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    let mut res = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, _c) in line.iter().enumerate() {
            if !visited.insert(Vec2::new(x as i32, y as i32)) {
                continue;
            }

            let area_name = line[x];

            let cells = find_cells(&input, Vec2::new(x as i32, y as i32), area_name);
            cells.iter().for_each(|&c| { visited.insert(c); });

            res.push(Area {
                cells,
            })
        }
    }

    res
}
fn get_area_cost(area: &Area) -> i32 {
    let all_dirs = vec![Vec2::new(1, 0), Vec2::new(-1, 0), Vec2::new(0, 1), Vec2::new(0, -1)];

    let circ: i32 =
        area.cells.iter()
            .map(|c| all_dirs.iter()
                .filter(|&d| !area.cells.contains(&(*d + *c)))
                .count() as i32)
            .sum();

    area.cells.len() as i32 * circ
}

fn get_area_cost_p2(area: &Area) -> i32 {
    let all_dirs = vec![Vec2::new(1, 0), Vec2::new(-1, 0), Vec2::new(0, 1), Vec2::new(0, -1)];

    let edge_cells: HashSet<(Vec2<i32>, Vec2<i32>)> = HashSet::from_iter(
        area.cells.iter()
            .flat_map(|&c| all_dirs.iter()
                .map(move |&d| (c + d, d))
                .filter(|(p, _)| !area.cells.contains(p)))
    );

    let edges = edge_cells
        .iter()
        .filter(|&(ec, ed)|
            !vec![Vec2::new(1, 0), Vec2::new(0, 1)].iter()
                .map(|&d| *ec + d)
                .any(|p| edge_cells.contains(&(p, *ed)))
        ).count() as i32;

    area.cells.len() as i32 * edges
}

pub fn part1(areas: &Vec<Area>) -> i32 {
    areas.iter()
        .map(|a| get_area_cost(a))
        .sum()
}

pub fn part2(areas: &Vec<Area>) -> i32 {
    areas.iter()
        .map(|a| get_area_cost_p2(a))
        .sum()
}

pub fn day12() {
    let input = fs::read_to_string("inputs/day12.txt")
        .expect("Could not read input");

    let areas = parse_input(&input);

    println!("Part 1: {}", part1(&areas));
    println!("Part 2: {}", part2(&areas));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let areas = parse_input(&input);
        assert_eq!(1930, part1(&areas));
    }

    #[test]
    fn test_part2_1() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let areas = parse_input(&input);
        assert_eq!(80, part2(&areas));
    }

    #[test]
    fn test_part2_2() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let areas = parse_input(&input);
        assert_eq!(236, part2(&areas));
    }

    #[test]
    fn test_part2_3() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let areas = parse_input(&input);
        assert_eq!(368, part2(&areas));
    }
}