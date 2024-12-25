use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
};

use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => panic!("Invalid operation name"),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::And => write!(f, "&"),
            Operation::Or => write!(f, "|"),
            Operation::Xor => write!(f, "^"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Node<'a> {
    name: &'a str,
    op: Operation,
    left: &'a str,
    right: &'a str,
}

type NodeValues<'a> = HashMap<&'a str, bool>;

fn parse_input(input: &str) -> (Vec<Node>, NodeValues) {
    let (init_values, nodes_list) = input.split_once("\n\n").expect("Invalid input");

    let node_values = HashMap::from_iter(
        init_values
            .lines()
            .filter_map(|l| l.trim().split_once(':'))
            .map(|(id, value)| (id, if value.trim() == "0" { false } else { true })),
    );

    let node_re = Regex::new(r"(\S+) (AND|OR|XOR) (\S+) -> (\S+)").expect("Bad regex");

    let nodes = nodes_list
        .lines()
        .filter_map(|l| node_re.captures(l))
        .map(|r| r.extract())
        .map(|(_, [left, op, right, name])| Node {
            name,
            left,
            right,
            op: op.into(),
        })
        .collect();

    (nodes, node_values)
}

fn resolve_values<'a>(nodes: &'a Vec<Node>, values: &'a NodeValues) -> NodeValues<'a> {
    let mut values = values.clone();

    let mut queue = nodes.clone();
    let nodes: HashMap<&str, Node> = HashMap::from_iter(nodes.iter().map(|n| (n.name, n.clone())));
    let mut depth = 0;

    while !queue.is_empty() {
        depth += 1;
        if depth > 1000 {
            // println!("Infinite loop");
            return values;
        }
        let node = queue.pop().expect("wtf");

        if values.contains_key(node.name) {
            continue;
        }

        let lvalue = values.get(node.left);
        let rvalue = values.get(node.right);

        if lvalue.is_none() || rvalue.is_none() {
            queue.push(node);
            if lvalue.is_none() {
                let lnode = nodes
                    .get(node.left)
                    .expect(format!("Invalid node {}", node.left).as_str());
                queue.push(*lnode);
            }
            if rvalue.is_none() {
                let rnode = nodes
                    .get(node.right)
                    .expect(format!("Invalid node {}", node.right).as_str());
                queue.push(*rnode);
            }
            continue;
        }

        let value = match node.op {
            Operation::And => lvalue.unwrap() & rvalue.unwrap(),
            Operation::Or => lvalue.unwrap() | rvalue.unwrap(),
            Operation::Xor => lvalue.unwrap() ^ rvalue.unwrap(),
        };
        values.insert(node.name, value);
    }
    values
}

fn part1(nodes: &Vec<Node>, values: &NodeValues) -> i64 {
    let values = resolve_values(&nodes, &values);
    let mut swapped_nodes = nodes.clone();
    swap_nodes(&mut swapped_nodes, "z05", "tst");
    println!("{:?}", swapped_nodes);
    for i in 0..45 {
        println!("OUT{i}");
        println!(
            "{}",
            show_graph(format!("z{i:0>2}").as_str(), &swapped_nodes, &values)
        );
    }

    let mut z_nodes: Vec<_> = nodes
        .iter()
        .map(|n| n.name)
        .filter(|n: &&str| n.starts_with("z"))
        .collect();
    z_nodes.sort();
    z_nodes.reverse();

    let mut res = 0i64;

    for z_node in z_nodes {
        let value = values
            .get(z_node)
            .expect(format!("Has not collected value for {z_node}").as_str());

        res <<= 1;

        if *value {
            res |= 1;
        }
    }

    res
}

fn show_graph(node: &str, nodes: &Vec<Node>, values: &NodeValues) -> String {
    let mut res = String::new();
    let mut stack = vec![(0, Some(node))];

    while !stack.is_empty() {
        let (depth, top) = stack.pop().unwrap();
        let margin = String::from("     |   ").repeat(depth);

        if top.is_none() {
            res += "\n";
            res += margin.as_str();
            continue;
        }

        let top = top.unwrap();

        let nop = nodes
            .iter()
            .find(|n| n.name == top)
            .map(|n| n.op)
            .unwrap_or(Operation::And);

        res += " - ";
        res += format!("{}{top}", nop).as_str();
        if let Some(val) = values.get(top) {
            if *val {
                res += " 1";
            } else {
                res += " 0";
            }
        } else {
            res += "  ";
        }

        if let Some(n) = nodes.iter().find(|n| n.name == top) {
            if depth < 500 {
                stack.push((depth + 1, Some(n.right)));
                stack.push((depth + 1, None));
                stack.push((depth + 1, Some(n.left)));
            }
        }
    }

    return res;
}

fn get_nodes<'a>(node: &'a str, nodes: &'a [Node]) -> Vec<&'a str> {
    let mut res = Vec::new();
    let mut queue = vec![node];

    while !queue.is_empty() {
        let n = queue.pop().unwrap();
        res.push(n);

        if let Some(node) = nodes.iter().find(|x| x.name == n) {
            queue.push(node.left);
            queue.push(node.right);
        }
    }

    res
}

fn test_bit<'a>(
    bit: i32,
    nodes: &'a Vec<Node>,
    init_values: &'a NodeValues,
) -> (bool, bool, bool, Vec<String>) {
    let x_node = format!("x{bit:0>2}");
    let y_node = format!("y{bit:0>2}");
    let z_node = format!("z{bit:0>2}");
    let z_carry_node = format!("z{:0>2}", bit + 1);

    let mut test_res = [true, true, true];
    let mut invalid_positive_nodes = Vec::new();

    // test set x
    let mut values = init_values.clone();
    values.insert(x_node.as_str(), true);

    let output = resolve_values(&nodes, &values);
    if !output.get(z_node.as_str()).unwrap_or(&false) {
        // potentially_bad_nodes.extend(pos_nodes.iter().map(|(n, _)| n.to_string()));
        invalid_positive_nodes.extend(
            output
                .iter()
                .filter(|(&n, &v)| n != x_node && v)
                .map(|(&n, _)| n),
        );
        test_res[0] = false;
    }

    // test set y
    let mut values = init_values.clone();
    values.insert(y_node.as_str(), true);

    let output = resolve_values(&nodes, &values);
    if !output.get(z_node.as_str()).unwrap_or(&false) {
        // potentially_bad_nodes.extend(pos_nodes.iter().map(|(n, _)| n.to_string()));
        invalid_positive_nodes.extend(
            output
                .iter()
                .filter(|(&n, &v)| n != y_node && v)
                .map(|(&n, _)| n),
        );
        test_res[1] = false;
    }

    // test set both
    let mut values = init_values.clone();
    values.insert(x_node.as_str(), true);
    values.insert(y_node.as_str(), true);

    let output = resolve_values(&nodes, &values);
    if !output.get(z_carry_node.as_str()).unwrap_or(&false) {
        // potentially_bad_nodes.extend(pos_nodes.iter().map(|(n, _)| n.to_string()));
        invalid_positive_nodes.extend(
            output
                .iter()
                .filter(|(&n, &v)| n != x_node && v)
                .map(|(&n, _)| n),
        );
        test_res[1] = false;
    }

    (
        test_res[0],
        test_res[1],
        test_res[2],
        invalid_positive_nodes
            .iter()
            .map(|s| s.to_string())
            .collect(),
    )
}

fn swap_nodes<'a>(nodes: &mut Vec<Node<'a>>, node0: &'a str, node1: &'a str) {
    for node in nodes {
        if node.name == node0 {
            node.name = node1;
        } else if node.name == node1 {
            node.name = node0;
        }
    }
}

fn part2(nodes: &Vec<Node>) -> String {
    let mut z_nodes: Vec<_> = nodes
        .iter()
        .map(|n| n.name)
        .filter(|n| n.starts_with("z"))
        .collect();
    z_nodes.sort();

    let x_nodes: Vec<_> = (0..z_nodes.len() - 1)
        .map(|i| format!("x{i:0>2}"))
        .collect();
    let y_nodes: Vec<_> = (0..z_nodes.len() - 1)
        .map(|i| format!("y{i:0>2}"))
        .collect();

    let init_values: HashMap<&str, bool> = HashMap::from_iter(
        x_nodes
            .iter()
            .map(|n| (n.as_str(), false))
            .chain(y_nodes.iter().map(|n| (n.as_str(), false))),
    );

    assert_eq!(x_nodes.len(), y_nodes.len());
    assert_eq!(z_nodes.len(), x_nodes.len() + 1);

    let mut swaps = HashSet::new();
    for i in 0..x_nodes.len() {
        let x_node = &x_nodes[i];
        let y_node = &y_nodes[i];
        let z_node = z_nodes[i];
        let z_carry_node = z_nodes[i + 1];

        let mut potentially_bad_nodes: Vec<String> = Vec::new();

        // test set x
        let mut values = init_values.clone();
        values.insert(x_node.as_str(), true);

        let output = resolve_values(&nodes, &values);
        let pos_nodes: Vec<_> = output.iter().filter(|(&n, &v)| n != x_node && v).collect();
        if !output.get(z_node).unwrap_or(&false) {
            println!("Bad at {x_node}: {pos_nodes:?}");
            println!("{}", show_graph(z_node, nodes, &output));
            for (n, _) in &pos_nodes {
                println!("{n}:");
                println!("{}", show_graph(n, nodes, &output));
            }
            potentially_bad_nodes.extend(pos_nodes.iter().map(|(n, _)| n.to_string()));
        }

        // test set y
        let mut values = init_values.clone();
        values.insert(y_node.as_str(), true);

        let output = resolve_values(&nodes, &values);
        let pos_nodes: Vec<_> = output.iter().filter(|(&n, &v)| n != y_node && v).collect();
        if !output.get(z_node).unwrap_or(&false) {
            println!("Bad at {y_node}: {pos_nodes:?}");
            potentially_bad_nodes.extend(pos_nodes.iter().map(|(n, _)| n.to_string()));
        }

        // test set both
        let mut values = init_values.clone();
        values.insert(y_node.as_str(), true);
        values.insert(x_node.as_str(), true);

        let output = resolve_values(&nodes, &values);
        let pos_nodes: Vec<_> = output
            .iter()
            .filter(|(&n, &v)| n != x_node && n != y_node && v)
            .collect();
        if !output.get(z_carry_node).unwrap_or(&false) {
            println!("Bad at {x_node} {y_node}: {pos_nodes:?}");
            potentially_bad_nodes.extend(pos_nodes.iter().map(|(n, _)| n.to_string()));
        }

        // Try swapping nodes around to find the correct answer
        let bad_node_set: HashSet<&str> =
            HashSet::from_iter(potentially_bad_nodes.iter().map(|n| n.as_str()));
        let nodes_in_output = get_nodes(z_node, &nodes);
        let nodes_in_carry = get_nodes(z_carry_node, &nodes);
        let nodes_in_output: Vec<_> = nodes_in_output
            .iter()
            .chain(nodes_in_carry.iter())
            .collect();

        for bad_on_node in bad_node_set {
            for bad_off_node in &nodes_in_output {
                if bad_on_node.starts_with("z") && bad_off_node.starts_with("z") {
                    continue;
                }

                if bad_off_node.starts_with("x") || bad_off_node.starts_with("y") {
                    continue;
                }

                if bad_on_node == **bad_off_node {
                    continue;
                }

                let mut swapped_nodes = nodes.clone();

                swap_nodes(&mut swapped_nodes, bad_on_node, bad_off_node);

                let (t1, t2, t3, _) = test_bit(i as i32, &swapped_nodes, &init_values);
                if t1 && t2 && t3 {
                    println!("Successfully swapped {bad_on_node} and {}", *bad_off_node);
                    swaps.insert((bad_on_node.to_string(), bad_off_node.to_string()));
                }
            }
        }
    }

    println!("Swaps: {swaps:?}");

    let mut si0 = 0;
    let mut si1 = 1;
    let mut si2 = 2;
    let mut si3 = 3;

    let mut best: Option<(Vec<i32>, [&(String, String); 4])> = None;

    let swaps: Vec<_> = swaps.iter().collect();
    loop {
        println!("Testing swap {si0}, {si1}, {si2}, {si3}");
        let swap_0 = &swaps[si0];
        let swap_1 = &swaps[si1];
        let swap_2 = &swaps[si2];
        let swap_3 = &swaps[si3];

        let mut test_swaps = nodes.clone();
        swap_nodes(&mut test_swaps, swap_0.0.as_str(), swap_0.1.as_str());
        swap_nodes(&mut test_swaps, swap_1.0.as_str(), swap_1.1.as_str());
        swap_nodes(&mut test_swaps, swap_2.0.as_str(), swap_2.1.as_str());
        swap_nodes(&mut test_swaps, swap_3.0.as_str(), swap_3.1.as_str());

        let failed_bits: Vec<_> = (0..44)
            .map(|bit| {
                let (r1, r2, r3, _) = test_bit(bit, &test_swaps, &init_values);
                (bit, r1 && r2 && r3)
            })
            .filter(|(_, b)| !b)
            .map(|(b, _)| b)
            .collect();

        if let Some((ref b, _)) = best {
            if failed_bits.len() < b.len() {
                best = Some((failed_bits.clone(), [swap_0, swap_1, swap_2, swap_3]));
            }
        } else {
            best = Some((failed_bits.clone().into(), [swap_0, swap_1, swap_2, swap_3]));
        }

        if failed_bits.is_empty() {
            let mut sw = vec![
                swap_0.clone().0.clone(),
                swap_0.clone().1.clone(),
                swap_1.clone().0.clone(),
                swap_1.clone().1.clone(),
                swap_2.clone().0.clone(),
                swap_2.clone().1.clone(),
                swap_3.clone().0.clone(),
                swap_3.clone().1.clone(),
            ];
            sw.sort();
            return sw.join(",");
        }

        if si3 < swaps.len() - 1 {
            si3 += 1;
        } else if si2 < si3 - 1 {
            si2 += 1;
            si3 = si2 + 1;
        } else if si1 < si2 - 1 {
            si1 += 1;
            si2 = si1 + 1;
            si3 = si2 + 1;
        } else if si0 < si1 - 1 {
            si0 += 1;
            si1 = si0 + 1;
            si2 = si1 + 1;
            si3 = si2 + 1;
        } else {
            break;
        }
    }

    println!("Best: {best:?}");

    "hej".into()
}

pub fn day24() {
    let input = fs::read_to_string("inputs/day24.txt").expect("Could not read input");

    let (nodes, values) = parse_input(&input);

    println!("Part 1: {}", part1(&nodes, &values));
    println!("Part 2: {}", part2(&nodes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

        let (nodes, values) = parse_input(&input);

        assert_eq!(2024, part1(&nodes, &values));
    }
}
