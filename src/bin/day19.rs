use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

fn parse(input: &str) -> Input {
    let mut it = input.lines();
    let towels = it
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim().to_owned())
        .collect();

    let designs = it.skip(1).map(|line| line.to_owned()).collect();

    Input { towels, designs }
}

#[derive(Debug, Default)]
struct TrieNode {
    leaf: bool,
    children: HashMap<char, Box<TrieNode>>,
}

impl TrieNode {
    fn insert(&mut self, s: &str) {
        let mut node = self;
        for c in s.chars() {
            node = node.children.entry(c).or_default();
        }
        node.leaf = true;
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day19.txt").unwrap();

    let input = parse(&input);
    println!("{input:?}");
    let trie = build_trie(&input);
    println!("{trie:#?}");
}

fn build_trie(input: &Input) -> TrieNode {
    let mut ret = TrieNode::default();

    for towel in &input.towels {
        ret.insert(towel);
    }

    ret
}
