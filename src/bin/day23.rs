use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct NodeInfo {
    index: u32,
    edges: HashSet<u32>,
}

#[derive(Debug)]
struct Graph {
    num_nodes: u32,
    edges: HashMap<u32, Vec<u32>>,
    name_per_node: HashMap<u32, String>,
}

fn parse(input: &str) -> Graph {
    let mut nodes = HashSet::new();
    let mut edges_str: HashMap<String, Vec<String>> = HashMap::new();
    input.lines().for_each(|line| {
        let line = line.trim();
        if line.is_empty() {
            return;
        }

        let mut it = line.split("-");
        let a = it.next().unwrap().to_owned();
        let b = it.next().unwrap().to_owned();
        nodes.insert(a.clone());
        nodes.insert(b.clone());

        let edges_a = edges_str.entry(a.clone()).or_default();
        edges_a.push(b.clone());
        let edges_b = edges_str.entry(b).or_default();
        edges_b.push(a);
    });

    let mut nodes_sorted = nodes.iter().collect::<Vec<_>>();
    nodes_sorted.sort();

    let mut idx_per_name = HashMap::new();
    let mut name_per_idx = HashMap::new();
    for (i, n) in nodes_sorted.into_iter().enumerate() {
        idx_per_name.insert(n.clone(), i as u32);
        name_per_idx.insert(i as u32 as u32, n.clone());
    }

    let mut edges_normalized = HashMap::new();

    for (n, edges) in edges_str.iter() {
        let mut edges = edges.iter().map(|n| idx_per_name[n]).collect::<Vec<_>>();
        edges.sort();
        edges_normalized.insert(idx_per_name[n], edges);
    }

    Graph {
        num_nodes: name_per_idx.len() as u32,
        edges: edges_normalized,
        name_per_node: name_per_idx,
    }
}

fn part1(graph: &Graph) -> u32 {
    (0..graph.num_nodes)
        .filter(|i| graph.name_per_node[i].starts_with("t")) // only count nodes starting with 't'
        .map(|current_node| {
            let edges = &graph.edges[&current_node];
            let mut count = 0;
            for (i, u) in edges.iter().cloned().enumerate() {
                if graph.name_per_node[&u].starts_with("t") && u < current_node {
                    // avoid double counting nodes with t
                    continue;
                }
                for v in &edges[i + 1..] {
                    if graph.name_per_node[v].starts_with("t") && *v < current_node {
                        continue;
                    }
                    if graph.edges[&u].contains(&v) {
                        // because current_node < u, and edges are sorted, we are good to count this triple
                        count += 1;
                    }
                }
            }
            count
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day23.txt").unwrap();
    let graph = parse(&input);

    let part1 = part1(&graph);
    println!("part 1: {part1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"
        kh-tc
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
        td-yn
        "#;

    #[test]
    fn example() {
        let graph = parse(EXAMPLE);
        let part1 = part1(&graph);
        assert_eq!(part1, 7);
    }
}
