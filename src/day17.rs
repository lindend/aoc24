use std::fs;

type Registers = (i64, i64, i64);

type State = (Registers, i32);
type Program = Vec<i32>;

fn parse_input(input: &str) -> (Registers, Program) {
    let mut lines = input.lines();

    let (_, reg_a) = lines.next().expect("Could not read Register A").split_once(":").unwrap();
    let (_, reg_b) = lines.next().expect("Could not read Register B").split_once(":").unwrap();
    let (_, reg_c) = lines.next().expect("Could not read Register C").split_once(":").unwrap();

    let reg_a_value: i64 = reg_a.trim().parse().unwrap();
    let reg_b_value: i64 = reg_b.trim().parse().unwrap();
    let reg_c_value: i64 = reg_c.trim().parse().unwrap();

    let registers = (reg_a_value, reg_b_value, reg_c_value);

    lines.next();

    let (_, opcodes) = lines.next().expect("Could not read program").split_once(":").unwrap();
    let program = opcodes
        .trim()
        .split(",")
        .filter_map(|c| c.parse().ok())
        .collect();

    (registers, program)
}

fn inc_pc(state: &State) -> State {
    (state.0, state.1 + 2)
}

fn set_pc(state: &State, pc: i32) -> State {
    (state.0, pc)
}

fn set_a(state: &State, a: i64) -> State {
    ((a, state.0.1, state.0.2), state.1)
}

fn set_b(state: &State, b: i64) -> State {
    ((state.0.0, b, state.0.2), state.1)
}

fn set_c(state: &State, c: i64) -> State {
    ((state.0.0, state.0.1, c), state.1)
}

fn get_a(state: &State) -> i64 {
    state.0.0
}

fn get_b(state: &State) -> i64 {
    state.0.1
}


fn get_c(state: &State) -> i64 {
    state.0.2
}


fn combo_op(state: &State, op: i32) -> i64 {
    match op {
        0..=3 => op as i64,
        4 => get_a(state),
        5 => get_b(state),
        6 => get_c(state),
        _ => panic!("Invalid combo operand")
    }
}

fn handle_opcode(state: &State, program: &Program) -> Option<(State, Option<i32>)> {
    let pc = state.1;
    if pc < 0 || pc >= program.len() as i32 {
        return None;
    }
    let pc = pc as usize;

    let mut output = None;

    let opcode = program[pc];
    let operand = program[pc + 1];
    let state = match opcode {
        0 => inc_pc(&set_a(&state, get_a(&state) / i64::pow(2, combo_op(&state, operand) as u32))),
        1 => inc_pc(&set_b(&state, get_b(&state) ^ operand as i64)),
        2 => inc_pc(&set_b(&state, combo_op(&state, operand) % 8)),
        3 => if get_a(&state) == 0 { inc_pc(&state) } else { set_pc(&state, operand) },
        4 => inc_pc(&set_b(&state, get_b(&state) ^ get_c(&state))),
        5 => {
            output = Some((combo_op(&state, operand) % 8) as i32);
            inc_pc(&state)
        }
        6 => inc_pc(&set_b(&state, get_a(&state) / i64::pow(2, combo_op(&state, operand) as u32))),
        7 => inc_pc(&set_c(&state, get_a(&state) / i64::pow(2, combo_op(&state, operand) as u32))),
        _ => panic!("Invalid opcode")
    };

    Some((state, output))
}

fn part1(registers: &Registers, program: &Program) -> Vec<i32> {
    let mut output = Vec::new();
    let mut state: State = (*registers, 0);

    loop {
        if let Some(res) = handle_opcode(&state, &program) {
            state = res.0;
            if let Some(out) = res.1 {
                output.push(out);
            }
        } else {
            break;
        }
    }

    output
}

fn part2(program: &Program) -> i64 {
    let mut resumes = vec![(0, program.len() - 1, 0)];

    loop {
        let (mut a, mut p, mut d) = resumes.pop().unwrap();

        loop {
            let tmp_a = a | d;
            let registers = (tmp_a, 0, 0);
            let o = part1(&registers, &program);

            if o[0] == program[p] {
                if p == 0 {
                    return tmp_a;
                }

                if d < 7 {
                    resumes.push((a, p, d + 1));
                }

                a = tmp_a << 3;
                d = 0;
                p -= 1;
                continue;
            }

            d += 1;
            if d == 8 {
                println!("Got {} digits right. {o:?} {a:#01x}", program.len() - p - 1);
                break;
            }
        }
    }
}

fn fmt_part1(out: &Vec<i32>) -> String {
    out.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(",")
}

pub fn day17() {
    let input = fs::read_to_string("inputs/day17.txt").expect("Could not read input file");

    let (registers, program) = parse_input(&input);

    println!("Part 1: {}", fmt_part1(&part1(&registers, &program)));
    println!("Part 2: {}", part2(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let (registers, program) = parse_input(&input);

        assert_eq!("4,6,3,5,6,3,5,2,1,0", fmt_part1(&part1(&registers, &program)));
    }

    #[test]
    fn test_p2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let (_, program) = parse_input(&input);

        assert_eq!(117440, part2(&program));
    }
}