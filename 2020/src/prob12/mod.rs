const INPUT: &[(char, i64)] = include!("input");

pub fn solve() {
    {
        fn delta(dir: i64) -> (i64, i64) {
            match dir {
                0 => (0, -1),
                1 => (1, 0),
                2 => (0, 1),
                3 => (-1, 0),
                _ => unreachable!(),
            }
        }

        let mut x = 0i64;
        let mut y = 0i64;
        let mut dir = 1;

        for (action, amt) in INPUT {
            match action {
                'N' => y -= amt,
                'S' => y += amt,
                'E' => x += amt,
                'W' => x -= amt,
                'F' => {
                    let (dx, dy) = delta(dir);
                    x += dx * amt;
                    y += dy * amt;
                }
                'L' => {
                    dir = (dir + 3 * amt / 90) % 4;
                }
                'R' => {
                    dir = (dir + 1 * amt / 90) % 4;
                }
                _ => unreachable!(),
            }
        }

        println!("Part 1: {}", x.abs() + y.abs());
    }

    {
        fn rot(dir: char) -> impl Fn(i64, i64) -> (i64, i64) {
            match dir {
                'R' => |x: i64, y: i64| {
                    (-y, x)
                },
                'L' => |x: i64, y: i64| {
                    (y, -x)
                },
                _ => unreachable!()
            }
        }

        let mut wx = 10i64;
        let mut wy = -1i64;
        let mut sx = 0i64;
        let mut sy = 0i64;

        for (action, amt) in INPUT {
            match action {
                'N' => wy -= amt,
                'S' => wy += amt,
                'E' => wx += amt,
                'W' => wx -= amt,
                'F' => {
                    sx += (wx) * amt;
                    sy += (wy) * amt;
                }
                'L' => {
                    for _ in 0..(amt / 90) {
                        let (dwx, dwy) = rot('L')(wx, wy);
                        wx = dwx;
                        wy = dwy;
                    }
                }
                'R' => {
                    for _ in 0..(amt / 90) {
                        let (dwx, dwy) = rot('R')(wx, wy);
                        wx = dwx;
                        wy = dwy;
                    }
                }
                _ => unreachable!(),
            }
        }

        println!("Part 2: {}", sx.abs() + sy.abs())
    }
}
