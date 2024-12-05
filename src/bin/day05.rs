use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rules {
    dependencies: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
}

fn parse(text: &str) -> Rules {
    let mut rules = Rules {
        dependencies: HashMap::new(),
        updates: vec![],
    };

    let mut parsing_dependencies = true;
    for line in text.lines() {
        if line.trim() == "" {
            parsing_dependencies = false;
            continue;
        }
        if parsing_dependencies {
            let mut nums = line.split("|").map(|s| s.parse().unwrap());
            let a = nums.next().unwrap();
            let b = nums.next().unwrap();
            rules
                .dependencies
                .entry(a)
                .or_insert(HashSet::new())
                .insert(b);
        } else {
            let nums = line.split(",").map(|s| s.parse().unwrap()).collect();
            rules.updates.push(nums);
        }
    }

    rules
}

fn main() {
    let input = std::fs::read_to_string("input/day05.txt").unwrap();
    let rules = parse(&input);
    println!("{rules:#?}");
}
