use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Link<'a> = (&'a str, &'a str);

fn parse_input(input: &str) -> Vec<Link> {
    input
        .lines()
        .filter_map(|l| l.trim().split_once('-'))
        .collect()
}

fn part1(links: &Vec<Link>) -> i32 {
    let mut link_map = HashMap::new();
    let mut link_set = HashSet::new();
    for (link_0, link_1) in links {
        link_set.insert(link_0);
        link_set.insert(link_1);
        link_map
            .entry(link_0)
            .and_modify(|l: &mut Vec<&str>| l.push(link_1))
            .or_insert(vec![link_1]);
        link_map
            .entry(link_1)
            .and_modify(|l| l.push(link_0))
            .or_insert(vec![link_0]);
    }

    let mut truples = HashSet::new();

    for link in link_set {
        let nodes = link_map.get(link).unwrap();
        for i in 0..nodes.len() - 1 {
            for j in i + 1..nodes.len() {
                let link_0 = &nodes[i];
                let link_1 = &nodes[j];
                if link_map.get(link_0).unwrap().contains(&link_1) {
                    let mut links = [link, link_0, link_1];
                    links.sort();
                    truples.insert(links);
                }
            }
        }
    }

    truples
        .iter()
        .filter(|t| t.iter().any(|l| l.starts_with("t")))
        .count() as i32
}

fn part2(links: &[Link]) -> String {
    let mut link_map = HashMap::new();
    let mut link_set = HashSet::new();
    for (link_0, link_1) in links {
        link_set.insert(link_0);
        link_set.insert(link_1);
        link_map
            .entry(link_0)
            .and_modify(|l: &mut Vec<&str>| l.push(link_1))
            .or_insert(vec![link_1]);
        link_map
            .entry(link_1)
            .and_modify(|l| l.push(link_0))
            .or_insert(vec![link_0]);
    }

    let mut largest_set = Vec::new();

    for link in link_set {
        let nodes = link_map.get(link).unwrap();
        let mut largest_link_sets: Vec<HashSet<&&str>> = Vec::new();

        for node in nodes {
            for ls in &mut largest_link_sets {
                let node_links = link_map.get(node).unwrap();
                if !ls.iter().all(|l| node_links.contains(l)) {
                    continue;
                }
                ls.insert(node);
            }
            let mut this_cluster = HashSet::new();
            this_cluster.insert(link);
            this_cluster.insert(node);
            largest_link_sets.push(this_cluster);
        }

        let largest = largest_link_sets.iter().max_by_key(|n| n.len()).unwrap();
        if largest.len() > largest_set.len() {
            largest_set = largest.iter().map(|&&s| s).collect();
        }
    }

    largest_set.sort();
    largest_set.join(",")
}

pub fn day23() {
    let input = fs::read_to_string("inputs/day23.txt").expect("Could not read input");
    let links = parse_input(&input);

    println!("Part 1: {}", part1(&links));
    println!("Part 2: {}", part2(&links));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "kh-tc
    qp-kh
    de-cg
    ka-co
    yn-aq
    qp-ub
    cg-tb
    vc-aq
    tb-ka
    wh-tc
    yn-cg
    kh-ub
    ta-co
    de-co
    tc-td
    tb-wq
    wh-td
    ta-ka
    td-qp
    aq-cg
    wq-ub
    ub-vc
    de-ta
    wq-aq
    wq-vc
    wh-yn
    ka-de
    kh-ta
    co-tc
    wh-qp
    tb-vc
    td-yn";

        let links = parse_input(&input);
        assert_eq!(7, part1(&links));
    }

    #[test]
    fn test_p2() {
        let input = "kh-tc
    qp-kh
    de-cg
    ka-co
    yn-aq
    qp-ub
    cg-tb
    vc-aq
    tb-ka
    wh-tc
    yn-cg
    kh-ub
    ta-co
    de-co
    tc-td
    tb-wq
    wh-td
    ta-ka
    td-qp
    aq-cg
    wq-ub
    ub-vc
    de-ta
    wq-aq
    wq-vc
    wh-yn
    ka-de
    kh-ta
    co-tc
    wh-qp
    tb-vc
    td-yn";

        let links = parse_input(&input);
        assert_eq!("co,de,ka,ta", part2(&links));
    }
}
