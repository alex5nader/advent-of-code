use itertools::Itertools;
use std::ops::Shl;

const INPUT: &[u64] = include!("input");

fn removal_count(range: &[u64]) -> usize {
    if range.last().unwrap() - range.first().unwrap() <= 3 {
        return 1usize.shl(range.len() - 2);
    }

    (1..(range.len()-1))
        .flat_map(|i| range[1..(range.len()-1)].iter().combinations(i))
        .map(|mut subslice| {
            subslice.insert(0, &range[0]);
            subslice.push(&range[range.len() - 1]);
            subslice
        })
        .filter(|subslice| {
            for i in 0..(subslice.len()-1) {
                if subslice[i+1] - subslice[i] > 3 {
                    return false;
                }
            }
            true
        })
        .count()
}

pub fn solve() {
    let mut input = INPUT.iter().cloned().sorted().collect_vec();
    input.insert(0, 0);
    input.push(input[input.len() - 1] + 3);

    let groups = input.iter().enumerate()
        .group_by(|(i, _)| {
            if *i == 0 || *i == input.len() - 1 {
                true
            } else {
                input[i + 1] - input[i - 1] > 3
            }
        });

    let mut total = 1;
    for (necessary, group) in &groups {
        if necessary {
            continue;
        }
        let group = group.collect_vec();

        let mut group2 = group.iter().map(|(_, v)| v).cloned().cloned().collect_vec();
        group2.insert(0, input[group[0].0-1]);
        group2.push(input[group[group.len()-1].0+1]);

        if necessary {
            continue;
        }
        total *= removal_count(&group2);
    }

    println!("{}", total);
}