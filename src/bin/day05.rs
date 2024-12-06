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
            rules.dependencies.entry(a).or_default().insert(b);
        } else {
            let nums = line.split(",").map(|s| s.parse().unwrap()).collect();
            rules.updates.push(nums);
        }
    }

    rules
}

fn update_valid(rule: &Rules, update: &[u32]) -> Result<(), (usize, usize)> {
    for (i, a) in update.iter().enumerate() {
        for (j, b) in update.iter().enumerate().skip(i) {
            if let Some(deps_b) = rule.dependencies.get(b) {
                if deps_b.contains(a) {
                    return Err((i, j));
                }
            }
        }
    }

    Ok(())
}

fn part1(rules: &Rules) -> u32 {
    let mut ret = 0;
    for update in &rules.updates {
        if update_valid(rules, update).is_ok() {
            ret += update[update.len() / 2];
        }
    }
    ret
}

fn part2(rules: &Rules) -> u32 {
    let mut ret = 0;
    for update in &rules.updates {
        if let Err((i, j)) = update_valid(rules, update) {
            // Brute foce. We could compute a topological ordering and sort the update vec by it, but this is good enough.
            let mut update = update.clone();
            update.swap(i, j);
            while let Err((i, j)) = update_valid(rules, &update) {
                update.swap(i, j);
            }
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
    let part2_res = part2(&rules);
    println!("part 2 result: {part2_res}");
}
