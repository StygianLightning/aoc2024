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

fn max_cliques_rec(
    graph: &Graph,
    current_set: HashSet<u32>,
    mut candidate_set: HashSet<u32>,
    mut excluded_set: HashSet<u32>,
    reported: &mut Vec<HashSet<u32>>,
) {
    if candidate_set.is_empty() && excluded_set.is_empty() {
        reported.push(current_set);
        return;
    }

    let vertices = candidate_set.iter().cloned().collect::<Vec<_>>();

    for vertex in vertices {
        let mut current_set = current_set.clone();
        current_set.insert(vertex);

        let neighbors = graph.edges[&vertex].iter().cloned().collect::<HashSet<_>>();
        let candidate_set_rec_call = candidate_set
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<_>>();
        let excluded_set_rec_call = excluded_set
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<_>>();

        max_cliques_rec(
            graph,
            current_set,
            candidate_set_rec_call,
            excluded_set_rec_call,
            reported,
        );

        candidate_set.remove(&vertex);
        excluded_set.insert(vertex);
    }
}

fn max_cliques(graph: &Graph) -> Vec<HashSet<u32>> {
    let mut ret = vec![];
    max_cliques_rec(
        graph,
        HashSet::new(),
        (0..graph.num_nodes).collect(),
        HashSet::new(),
        &mut ret,
    );

    ret
}

fn part2(graph: &Graph) -> String {
    let max_cliques = max_cliques(graph);
    let max_clique = max_cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap();
    let mut sorted = max_clique
        .iter()
        .map(|x| graph.name_per_node[x].clone())
        .collect::<Vec<_>>();
    sorted.sort();
    let mut output = String::new();
    for s in &sorted {
        output += s;
        output += ",";
    }

    output = output.strip_suffix(",").unwrap().to_owned();

    output
}

fn main() {
    let input = std::fs::read_to_string("input/day23.txt").unwrap();
    let graph = parse(&input);

    let part1 = part1(&graph);
    println!("part 1: {part1}");

    let part2 = part2(&graph);
    println!("part 2: {part2:?}");
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
