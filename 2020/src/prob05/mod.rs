use std::collections::HashSet;
use itertools::Itertools;

const INPUT: &[(&[u8], &[u8])] = include!("input");

fn search(s: &[u8], min: usize, max: usize, left: u8, right: u8) -> usize {
    if s.len() == 0 {
        min
    } else {
        let mid = (min + max - 1) / 2;
        if s[0] == left {
            search(&s[1..], min, mid, left, right)
        } else {
            search(&s[1..], mid + 1, max, left, right)
        }
    }
}

fn to_id((row, col): (usize, usize)) -> usize {
    row * 8 + col
}

pub fn solve() {
    let possible_ids = (0..=127).cartesian_product(0..=7)
        .map(to_id)
        .collect::<HashSet<_>>();

    let found_ids = INPUT.iter()
        .map(|(row, col)| {
            (
                search(&row, 0, 127, b'F', b'B'),
                search(&col, 0, 7, b'L', b'R'),
            )
        })
        .map(to_id)
        .collect::<HashSet<_>>();

    let highest_id = found_ids
        .iter()
        .max()
        .unwrap();

    println!("Part A: {}", highest_id);

    let missing_ids = possible_ids.difference(&found_ids).collect::<HashSet<_>>();

    let my_seat_id = missing_ids
        .into_iter()
        .find(|id| found_ids.contains(&(**id+1)) && found_ids.contains(&(**id-1)))
        .unwrap();

    println!("Part B: {}", my_seat_id);
}
