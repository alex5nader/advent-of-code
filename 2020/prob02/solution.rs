use text_io::try_scan;

const INPUT: &[u8] = include_bytes!("input");

fn read_line<I: Iterator<Item=u8>>(input: &mut I) -> Result<(usize, usize, char, Vec<char>), Box<dyn std::error::Error>> {
    let min: usize;
    let max: usize;
    let target: char;
    let passwd: String;
    try_scan!(*input => "{}-{} {}: {}\n", min, max, target, passwd);
    Ok((min, max, target, passwd.chars().collect()))
}

fn main() {
    let mut input = INPUT.iter().cloned();

    let mut valid_a_count = 0;
    let mut valid_b_count = 0;
    while let Ok((min, max, target, passwd)) = read_line(&mut input) {
        if (min..=max).contains(&passwd.iter().filter(|c| **c == target).count()) {
            valid_a_count += 1;
        }

        let min = min - 1;
        let max = max - 1;
        let min_ok = min < passwd.len() && passwd[min] == target;
        let max_ok = max < passwd.len() && passwd[max] == target;
        if min_ok != max_ok {
            valid_b_count += 1;
        }
    }
    println!("Part A: {}", valid_a_count);
    println!("Part B: {}", valid_b_count);
}
