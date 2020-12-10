use std::collections::{HashMap, HashSet};
use std::str::FromStr;

macro_rules! checks {
    ($($name:expr => |$param:ident| $body:expr),* $(,)?) => {
        vec![$(
            ($name, Box::new(|$param: &'static str| $body) as Box<dyn Fn(&'static str) -> bool>)
        ),*].into_iter().collect::<HashMap<_, _>>()
    }
}

const INPUT: &[&[(&'static str, &'static str)]] = include!("input");

pub fn solve() {
    let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter().collect::<HashSet<_>>();

    fn in_range(s: &str, min: i64, max: i64) -> bool {
        i64::from_str(s).map(|x| min <= x && x <= max).unwrap_or(false)
    }

    let checks = checks! {
        "byr" => |e| e.len() == 4 && in_range(e, 1920, 2002),
        "iyr" => |e| e.len() == 4 && in_range(e, 2010, 2020),
        "eyr" => |e| e.len() == 4 && in_range(e, 2020, 2030),
        "hgt" => |e| {
            let (val, unit) = e.split_at(e.len()-2);
            match unit {
                "cm" => in_range(val, 150, 193),
                "in" => in_range(val, 59, 76),
                _ => false
            }
        },
        "hcl" => |e| {
            let (hash, val) = e.split_at(1);
            hash == "#" && val.len() == 6 && i64::from_str_radix(val, 16).is_ok()
        },
        "ecl" => |e| valid_ecl.contains(e),
        "pid" => |e| e.len() == 9 && u64::from_str(e).is_ok(),
    };

    let passports: Vec<HashMap<&'static str, &'static str>> = INPUT.iter()
        .map(|p| p.iter().cloned().collect())
        .collect();

    let (count_a, count_b) = passports.iter()
        .map(|passport| {
            checks
                .iter()
                .try_fold((1, 1), |(valid_a, valid_b), (key, check)| {
                    if !passport.contains_key(key) {
                        Err((0, 0))
                    } else if !check(&passport[key]) {
                        Ok((valid_a, 0))
                    } else {
                        Ok((valid_a, valid_b))
                    }
                }).unwrap_or((0, 0))
        })
        .fold((0, 0), |(count_a, count_b), (cur_a, cur_b)| (count_a + cur_a, count_b + cur_b));

    println!("Part A: {}", count_a);
    println!("Part B: {}", count_b);
}
