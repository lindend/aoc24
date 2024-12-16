use crate::util::vec2::Vec2;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

enum Input {
    Up,
    Down,
    Left,
    Right,
}

impl Input {
    fn get_move(&self) -> Vec2<i32> {
        match self {
            Input::Up => Vec2::new(0, -1),
            Input::Down => Vec2::new(0, 1),
            Input::Left => Vec2::new(-1, 0),
            Input::Right => Vec2::new(1, 0)
        }
    }
}

impl TryFrom<char> for Input {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            _ => Err(())
        }
    }
}

type Inputs = Vec<Input>;

#[derive(Clone, PartialEq)]
enum Obj {
    Robot,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
}

impl Obj {
    fn new(c: char) -> Option<Self> {
        match c {
            '#' => Some(Self::Wall),
            'O' => Some(Self::Box),
            '@' => Some(Self::Robot),
            _ => None,
        }
    }
}

impl Display for Obj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Obj::Robot => write!(f, "@"),
            Obj::Box => write!(f, "O"),
            Obj::BoxLeft => write!(f, "["),
            Obj::BoxRight => write!(f, "]"),
            Obj::Wall => write!(f, "#"),
        }
    }
}
type Map = HashMap<Vec2<i32>, Obj>;

fn move_obj(map: &mut Map, from: Vec2<i32>, to: Vec2<i32>) {
    let obj = map.remove(&from).expect("Can't move nothing!");
    map.insert(to, obj);
}

fn parse_input(input: &str) -> (Map, Inputs) {
    let (map_input, move_input) = input.split_once("\n\n").expect("Could not split input in two");

    let map = map_input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .map(move |(x, c)| (Vec2::new(x as i32, y as i32), Obj::new(c))))
        .filter_map(|(p, e)| match e {
            Some(obj) => Some((p, obj)),
            None => None,
        });

    let map = Map::from_iter(map);

    let moves: Inputs = move_input.chars().filter_map(|c| c.try_into().ok()).collect();
    (map, moves)
}

fn widen_pos(pos: Vec2<i32>) -> Vec2<i32> {
    Vec2::new(pos.x * 2, pos.y)
}

fn widen_map(map: &Map) -> Map {
    let right = Vec2::new(1, 0);
    Map::from_iter(map.iter()
        .flat_map(move |(&p, o)| match o {
            Obj::Robot => vec![(widen_pos(p), o.clone())],
            Obj::Wall => vec![(widen_pos(p), o.clone()), (widen_pos(p) + right, o.clone())],
            Obj::Box => vec![(widen_pos(p), Obj::BoxLeft), (widen_pos(p) + right, Obj::BoxRight)],
            _ => vec![]
        })
    )
}

fn move_object(pos: Vec2<i32>, dir: Vec2<i32>, map: &mut Map) -> bool {
    let target_pos = pos + dir;
    let at_pos = map.get(&target_pos);
    let did_move = match at_pos {
        None => {
            true
        }
        Some(Obj::Box) => {
            if move_object(target_pos, dir, map) {
                true
            } else {
                false
            }
        }
        Some(Obj::BoxLeft) =>
            if dir.x == 1 {
                move_object(target_pos + Vec2::new(1, 0), dir, map) && move_object(target_pos, dir, map)
            } else {
                move_object(target_pos, dir, map) && move_object(target_pos + Vec2::new(1, 0), dir, map)
            },
        Some(Obj::BoxRight) =>
            if dir.x == -1 {
                move_object(target_pos + Vec2::new(-1, 0), dir, map) && move_object(target_pos, dir, map)
            } else {
                move_object(target_pos, dir, map) && move_object(target_pos + Vec2::new(-1, 0), dir, map)
            },
        _ => false
    };

    if did_move {
        move_obj(map, pos, target_pos);
    }
    did_move
}

fn gps_coord(pos: Vec2<i32>) -> i32 {
    pos.x + 100 * pos.y
}

fn part1(map: &Map, inputs: &Inputs) -> i32 {
    let mut map = map.clone();
    let mut robot_pos = *map.iter().find(|(pos, obj)| **obj == Obj::Robot).expect("Could not find robot").0;
    for mv in inputs {
        let dir = mv.get_move();
        if move_object(robot_pos, dir, &mut map) {
            robot_pos = robot_pos + dir;
        }
    }

    map.iter()
        .filter(|(_, obj)| **obj == Obj::Box)
        .map(|(pos, _)| gps_coord(*pos))
        .sum()
}

fn part2(map: &Map, inputs: &Inputs) -> i32 {
    let mut map = widen_map(map);
    let mut robot_pos = *map.iter().find(|(pos, obj)| **obj == Obj::Robot).expect("Could not find robot").0;
    for mv in inputs {
        let mut tmp_map = map.clone();
        let dir = mv.get_move();
        if move_object(robot_pos, dir, &mut tmp_map) {
            robot_pos = robot_pos + dir;
            map = tmp_map;
        }
    }

    map.iter()
        .filter(|(_, obj)| **obj == Obj::BoxLeft)
        .map(|(pos, _)| gps_coord(*pos))
        .sum()
}


pub fn day15() {
    let input = fs::read_to_string("inputs/day15.txt").expect("Could not read input");
    let (map, inputs) = parse_input(&input);

    println!("Part 1: {}", part1(&map, &inputs));
    println!("Part 2: {}", part2(&map, &inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let (map, inputs) = parse_input(&input);

        assert_eq!(10092, part1(&map, &inputs));
    }

    #[test]
    fn test_p2() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let (map, inputs) = parse_input(&input);

        assert_eq!(9021, part2(&map, &inputs));
    }
}