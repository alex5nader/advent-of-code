use itertools::Itertools;
use crate::vec2d::Vec2d;

const INPUT: &[&[u8]] = include!("input");

#[derive(Clone, PartialEq, Eq)]
enum Cell {
    Floor,
    Empty,
    Occupied
}

use Cell::*;

pub fn solve() {
    let cells: Vec2d<Cell> = INPUT.iter()
        .map(|row| row.iter().map(|x| {
            match x {
                b'.' => Floor,
                b'L' => Empty,
                _ => unreachable!(),
            }
        })).collect();

    let count_1 = {
        let mut cells = cells.clone();
        loop {
            let mut new_cells = cells.clone();

            fn adj_1(seats: &Vec2d<Cell>, (i, j): (usize, usize)) -> usize {
                let mut count = 0;

                for (di, dj) in (-1isize..=1).cartesian_product(-1isize..=1) {
                    if di == 0 && dj == 0 {
                        continue;
                    }

                    let i = i as isize + di;
                    let j = j as isize + dj;

                    if i < 0 || i as usize >= seats.row_count() || j < 0 || j as usize >= seats.column_count() {
                        continue;
                    }

                    if seats[i as usize][j as usize] == Occupied {
                        count += 1;
                    }
                }

                count
            }

            for idx in cells.indices() {
                let adj = adj_1(&cells, idx);

                match cells[idx] {
                    Empty if adj == 0 => new_cells[idx] = Occupied,
                    Occupied if adj >= 4 => new_cells[idx] = Empty,
                    _ => {},
                }
            }

            if new_cells == cells {
                break cells.items().filter(|cell| **cell == Occupied).count();
            }

            cells = new_cells;
        }
    };

    println!("Part 1: {}", count_1);

    let count_2 = {
        let mut cells = cells.clone();
        loop {
            let mut new_cells = cells.clone();

            fn adj_2(seats: &Vec2d<Cell>, (i, j): (usize, usize)) -> usize {
                let mut count = 0;
                'directions: for (di, dj) in (-1isize..=1).cartesian_product(-1isize..=1) {
                    if di == 0 && dj == 0 {
                        continue;
                    }

                    let advance = |i: usize, j: usize| {
                        let i = i as isize + di;
                        let j = j as isize + dj;

                        if i < 0 || i as usize >= seats.len() || j < 0 || j as usize >= seats[i as usize].len() {
                            None
                        } else {
                            Some((i as usize, j as usize))
                        }
                    };

                    let mut i = i;
                    let mut j = j;
                    while let Some((_i, _j)) = advance(i, j) {
                        i = _i;
                        j = _j;

                        if seats[i][j] == Occupied {
                            count += 1;
                            continue 'directions;
                        } else if seats[i][j] == Empty {
                            continue 'directions;
                        }
                    }
                }
                count
            }

            for idx in cells.indices() {
                let adj = adj_2(&cells, idx);

                match cells[idx] {
                    Empty if adj == 0 => new_cells[idx] = Occupied,
                    Occupied if adj >= 5 => new_cells[idx] = Empty,
                    _ => {},
                }
            }

            if new_cells == cells {
                break cells.items().filter(|cell| **cell == Occupied).count();
            }

            cells = new_cells;
        }
    };

    println!("Part 2: {}", count_2);
}