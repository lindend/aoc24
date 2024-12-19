use crate::util::vec2::Vec2;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

pub fn print_grid<TV: Display>(grid: &HashMap<Vec2<i32>, TV>, size: Vec2<i32>) -> String {
    let mut res = String::new();
    res.reserve(((size.x + 2) * size.y) as usize);

    for y in 0..size.y {
        for x in 0..size.x {
            res.push_str(
                &grid
                    .get(&Vec2::new(x, y))
                    .map(|n| n.to_string())
                    .unwrap_or(String::from(".")))
        }
        res.push('\n');
    }

    res
}

pub fn print_grid_hashset(grid: &HashSet<Vec2<i32>>, size: Vec2<i32>) -> String {
    let mut res = String::new();
    res.reserve(((size.x + 2) * size.y) as usize);

    for y in 0..size.y {
        for x in 0..size.x {
            res.push_str(if grid.contains(&Vec2::new(x, y)) { "#" } else { "." })
        }
        res.push('\n');
    }

    res
}