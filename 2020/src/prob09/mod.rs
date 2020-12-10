use std::collections::HashSet;
use itertools::Itertools;

const INPUT: &[i64] = include!("input");

pub fn solve() {
    let target = {
        let mut start = 0;

        'find_target:
        loop {
            let end = start+25;
            let window = &INPUT[start..end];
            let target = INPUT[end];

            let mut nums = HashSet::new();

            for x in window {
                if nums.contains(&(target - x)) {
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
            if INPUT[start..=end].iter().sum::<i64>() == target {
                let (min, max) = INPUT[start..=end].iter().minmax().into_option().unwrap();
                println!("Part B: {}..{} => {}", start, end, min + max);
                break 'find_minmax;
            }
        }
    }
}
