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

    fn child(&self, c: char) -> Option<&Box<TrieNode>> {
        self.children.get(&c)
    }
}

fn check_design(design: &str, mut root: &TrieNode) -> usize {
    let mut paths = vec![0; design.len() + 1];
    paths[0] = 1;

    for i in 0..design.len() {
        let mut node = root;
        if paths[i] == 0 {
            continue;
        }

        for (idx, c) in design.char_indices().skip(i) {
            if let Some(n) = node.child(c) {
                node = n;
                if node.leaf {
                    paths[idx + 1] += paths[i];
                }
            } else {
                break;
            }
        }
    }

    *paths.last().unwrap()
}

fn part1(designs: &[String], trie: &TrieNode) -> usize {
    designs.iter().filter(|d| check_design(d, trie) > 0).count()
}

fn part2(designs: &[String], trie: &TrieNode) -> usize {
    designs.iter().map(|d| check_design(d, trie)).sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day19.txt").unwrap();

    let input = parse(&input);
    println!("{input:?}");
    let trie = build_trie(&input);
    println!("{trie:#?}");

    let part1_res = part1(&input.designs, &trie);
    println!("part 1: {part1_res}");

    let part2_res = part2(&input.designs, &trie);
    println!("part 2: {part2_res}");
}

fn build_trie(input: &Input) -> TrieNode {
    let mut ret = TrieNode::default();

    for towel in &input.towels {
        ret.insert(towel);
    }

    ret
}
