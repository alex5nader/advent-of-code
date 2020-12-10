use std::collections::{HashMap, HashSet};

const INPUT: &[&str] = include!("input");

pub fn solve() {
    // container -> contained -> amount contained
    let mut contains: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    for rule in INPUT {
        let mut split = rule.split("contain");

        let container = split.next().unwrap().trim();
        let contained = split.next().unwrap().split(",");

        for contained in contained {
            let contained = contained.trim();

            if &contained[0..2] != "no" {
                let amt = contained[0..1].parse::<usize>().unwrap();
                let color = contained[1..].trim();

                contains.entry(container).or_default().insert(color, amt);
            }
        }
    }

    {
        let mut visited = HashSet::new();

        let mut to_visit = Vec::new();
        to_visit.push("shiny gold bag");

        while let Some(target) = to_visit.pop() {
            if !visited.insert(target) {
                continue;
            }

            for (container, contained) in &contains {
                if contained.contains_key(&target) {
                    to_visit.push(container);
                }
            }
        }

        println!("Part A: {}", visited.len() - 1);
    }

    {
        fn get_amt(contains: &HashMap<&str, HashMap<&str, usize>>, target: &str) -> usize {
            1 + contains.get(target).map_or(0, |contained| {
                contained.iter().map(|(value, amt)| amt * get_amt(contains, value)).sum()
            })
        }

        println!("Part B: {}", get_amt(&contains, "shiny gold bag") - 1);
    }
}
