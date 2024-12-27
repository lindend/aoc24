// use crate::util::vec2::Vec2;
// use std::cmp::Ordering;
// use std::collections::{BinaryHeap, HashMap, HashSet};
// use std::fmt::{Display, Formatter};

// #[derive(PartialEq, Eq, Hash, Clone)]
// struct Edge {
//     target: Node,
//     link: Box<dyn Link + Clone>,
// }

// impl Edge {
//     pub fn traverse(&mut self) -> (i32, Node) {
//         (self.link.traverse(&self.target), self.target.clone())
//     }
// }

// #[derive(PartialEq, Eq, Hash, Clone)]
// struct Node {
//     id: String,
//     edges: Vec<Edge>,
// }

// struct CostLink {
//     cost: i32,
// }

// trait Link {
//     fn traverse(&mut self, target: &Node) -> (i32, Vec<Node>);
// }

// impl Link for CostLink {
//     fn traverse(&mut self, _: &Node) -> (i32, Vec<Node>) {
//         (self.cost, Vec::new())
//     }
// }

// #[derive(PartialEq, Eq)]
// struct State<'a> {
//     cost: i32,
//     node: Node,
//     prev_node: Node,
//     targets: &'a [&'a Node],
//     path: Vec<Node>,
// }

// impl<'a> Ord for State<'a> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//     }
// }

// impl<'a> PartialOrd for State<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// struct Metagraph {
//     current_node: Node,
// }

// impl Metagraph {
//     pub fn shortest_path(&mut self, start: &Node, targets: &Vec<&Node>) -> (i32, Vec<Node>) {
//         let mut heads = BinaryHeap::new();
//         heads.push(State {
//             cost: 0,
//             node: start.clone(),
//             prev_node: start.clone(),
//             targets: targets.as_slice(),
//             path: Vec::new(),
//         });

//         let mut visited = HashMap::new();

//         loop {
//             let head = heads.pop().expect("No path");

//             visited.insert(head.node, head.cost);

//             let mut targets = head.targets;

//             assert!(!targets.is_empty());

//             let mut cost = head.cost;
//             let mut path = head.path.clone();

//             if head.node == *targets[0] {
//                 targets = &targets[1..];
//             }

//             if targets.is_empty() {
//                 return (head.cost, head.path);
//             }

//             for edge in head.node.edges {
//                 let (traverse_cost, new_node) = edge.traverse();
//                 let cost = cost + traverse_cost;

//                 let mut path = path.clone();

//                 path.push(new_node.clone());

//                 heads.push(State {
//                     cost,
//                     node: new_node,
//                     prev_node: head.node,
//                     targets,
//                     path,
//                 });
//             }
//         }
//     }
// }

// impl Link for Metagraph {
//     fn traverse(&mut self, target: &Node) -> (i32, Vec<Node>) {
//         let (cost, path) = self.shortest_path(&self.current_node, &vec![target]);
//         self.current_node = target.clone();
//         (cost, path)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1() {
//         let dp = Metagraph::new();
//         let left = dp.add_node("<");
//         let up = dp.add_node("^");
//         let right = dp.add_node(">");
//         let down = dp.add_node("v");
//         let dp_a = dp.add_node("A");

//         dp.add_edge(left, down, CostLink { cost: 1 });
//         dp.add_edge(down, right, CostLink { cost: 1 });
//         dp.add_edge(down, up, CostLink { cost: 1 });
//         dp.add_edge(up, dp_a, CostLink { cost: 1 });
//         dp.add_edge(right, dp_a, CostLink { cost: 1 });

//         let kp = Metagraph::new();
//         kp.add_node("0");
//         kp.add_node("1");
//         kp.add_node("2");
//         kp.add_node("3");
//         kp.add_node("4");
//         kp.add_node("5");
//         kp.add_node("6");
//         kp.add_node("A");

//         kp.add_edge(kp_0, kp_1, dp.target_link(vec![right, dp_a]));

//         mg.shortest_path();
//     }
// }
