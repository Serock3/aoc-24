use std::collections::BTreeSet;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_graph(input);
    let unique: Vec<BTreeSet<&str>> = graph
        .iter()
        .flat_map(|(node, nodes_1)| {
            nodes_1.iter().flat_map(|node_1| {
                let nodes_2 = graph.get(node_1).unwrap();
                nodes_2
                    .iter()
                    .filter(|&&node_2| graph.get(node_2).unwrap().iter().contains(node))
                    .map(|&node_2| [node, node_1, node_2])
                    .filter(|l| l.iter().any(|n| n.starts_with('t')))
                    .map(BTreeSet::from)
            })
        })
        .unique()
        .collect_vec();
    Some(unique.len())
}

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for (a, b) in input.lines().map(|line| line.split_once('-').unwrap()) {
        graph
            .entry(a)
            .and_modify(|connected_nodes| connected_nodes.push(b))
            .or_insert(vec![b]);
        graph
            .entry(b)
            .and_modify(|connected_nodes| connected_nodes.push(a))
            .or_insert(vec![a]);
    }
    graph
}

pub fn part_two(input: &str) -> Option<String> {
    let graph = parse_graph(input);

    let mut all_connected_sets = HashSet::new();
    for node in graph.keys() {
        search(node, [*node].into(), &mut all_connected_sets, &graph)
    }
    let max_set = all_connected_sets
        .into_iter()
        .max_by_key(BTreeSet::len)
        .unwrap();

    Some(max_set.iter().join(","))
}

fn search<'a>(
    node: &str,
    current_connected_set: BTreeSet<&'a str>,
    all_connected_sets: &mut HashSet<BTreeSet<&'a str>>,
    graph: &HashMap<&'a str, Vec<&'a str>>,
) {
    if all_connected_sets.insert(current_connected_set.clone()) {
        for neighbor in graph.get(&node).unwrap() {
            if !current_connected_set.contains(neighbor) {
                let next = graph.get(neighbor).unwrap();
                if current_connected_set.iter().all(|node| next.contains(node)) {
                    let mut current_connected_set = current_connected_set.clone();
                    current_connected_set.insert(neighbor);
                    search(neighbor, current_connected_set, all_connected_sets, graph);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "kh-tc
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

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
