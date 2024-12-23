use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, Vec<String>>,
}

fn parse(input: &str) -> Graph {
    let mut graph = Graph::default();
    input.lines().for_each(|line| {
        let mut it = line.split("-");
        let a = it.next().unwrap().to_owned();
        let b = it.next().unwrap().to_owned();
        graph.nodes.insert(a.clone());
        graph.nodes.insert(b.clone());

        let edges_a = graph.edges.entry(a.clone()).or_default();
        edges_a.push(b.clone());
        let edges_b = graph.edges.entry(b).or_default();
        edges_b.push(a);
    });

    graph
}

fn main() {
    let input = std::fs::read_to_string("input/day23.txt").unwrap();

    let graph = parse(&input);
    println!("{graph:#?}");
}
