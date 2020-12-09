use std::collections::HashSet;
use itertools::Itertools;
use std::cmp::Ordering;

const INPUT: &[u64] = include!("input");

fn main() {
    let target = {
        let mut start = 0;

        'find_target:
        loop {
            let target = start+25;
            let window = &INPUT[start..target];
            let target = INPUT[target];

            let mut nums = HashSet::new();

            for x in window {
                if *x < target && nums.contains(&(target - x)) {
                    start += 1;
                    continue 'find_target;
                }
                nums.insert(x);
            }

            break target;
        }
    };
    println!("Part A: {}", target);

    'find_minmax:
    for start in 0..INPUT.len() {
        for end in start..INPUT.len() {
            match target.cmp(&INPUT[start..=end].iter().sum::<u64>()) {
                Ordering::Greater => continue,
                Ordering::Less => continue 'find_minmax,
                Ordering::Equal => {
                    let (min, max) = INPUT[start..=end].iter().minmax().into_option().unwrap();
                    println!("Part B: {}", min + max);
                    break 'find_minmax;
                }
            }
        }
    }
}
