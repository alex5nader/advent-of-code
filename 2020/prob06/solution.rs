use itertools::Itertools;

const INPUT: &[&[&[u8]]] = include!("input");

fn main() {
    let any_yes_count = INPUT.iter()
        .map(|g| {
            g.iter()
                .flat_map(|person| person.iter())
                .unique()
                .count()
        })
        .sum::<usize>();

    let all_yes_count = INPUT.iter()
        .map(|g| {
            g.iter()
                .flat_map(|person| person.iter())
                .unique()
                .filter(|ans| g.iter().all(|person| person.contains(ans)))
                .count()
        })
        .sum::<usize>();

    println!("Part A: {}", any_yes_count);
    println!("Part B: {}", all_yes_count);
}
