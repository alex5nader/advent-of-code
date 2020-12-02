use text_io::try_scan;
use std::io::Read;

const INPUT: &[u8] = include_bytes!("input");

fn read_line<I: Iterator<Item=u8>>(input: &mut I) -> Result<(usize, usize, char, String), Box<dyn std::error::Error>> {
    let min: usize;
    let max: usize;
    let target: char;
    let passwd: String;
    try_scan!(*input => "{}-{} {}: {}\n", min, max, target, passwd);
    Ok((min, max, target, passwd))
}

fn main() {
    let mut input = INPUT.iter().cloned();
    let mut valid_count = 0;

    while let Ok((min, max, target, passwd)) = read_line(&mut input) {
        if (min..=max).contains(&passwd.chars().filter(|c| *c == target).count()) {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}
