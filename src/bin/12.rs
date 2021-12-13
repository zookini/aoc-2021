use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let graph = include_str!("../../input/12.txt")
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .into_group_map();

    println!("Part 1: {}", walk(&graph, "start", &mut vec![], true));
    println!("Part 2: {}", walk(&graph, "start", &mut vec![], false));
}

fn walk<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, node: &'a str, path: &mut Vec<&'a str>, revisited: bool) -> usize {
    let dupe = node.chars().all(|c| c.is_lowercase()) && path.contains(&node);

    if (node == "start" || revisited) && dupe {
        0
    } else if node == "end" {
        1
    } else {
        path.push(node);
        let result = graph[node].iter().map(|child| walk(graph, child, path, dupe || revisited)).sum();
        path.pop();
        result
    }
}
