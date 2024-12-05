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

fn update_valid(rule: &Rules, update: &[u32]) -> bool {
    for (i, a) in update.iter().enumerate() {
        for b in update[i + 1..].iter() {
            if let Some(deps_b) = rule.dependencies.get(&b) {
                if deps_b.contains(&a) {
                    return false;
                }
            }
        }
    }

    return true;
}

fn part1(rules: &Rules) -> u32 {
    let mut ret = 0;
    for update in &rules.updates {
        if update_valid(rules, update) {
            ret += update[update.len() / 2];
        }
    }
    ret
}

fn main() {
    let input = std::fs::read_to_string("input/day05.txt").unwrap();
    let rules = parse(&input);
    let part1_res = part1(&rules);
    println!("part 1 result: {part1_res}");
}
