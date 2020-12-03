const INPUT: [&'static str; 323] = include!("input");

fn main() {
    let map = INPUT.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let w =  (&map[0]).len();
    let h = map.len();

    let count = |dx: usize, dy: usize| {
        std::iter::successors(Some(0usize), |x| x.checked_add(dx))
            .zip((0..h).step_by(dy))
            .filter(|(x, y)| (&map[*y])[*x % w] == '#')
            .count()
    };

    println!("Part A: {}", count(3, 1));
    println!("Part B: {}", count(1, 1) * count(3, 1) * count(5, 1) * count(7, 1) * count(1, 2));
}
