static INPUT: &str = include_str!("input");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize
}

fn read_galaxies(larger_expansion: bool) -> Vec<Coord> {
    let mut galaxies = Vec::with_capacity(500);
    let mut missing_rows = 0;
    let mut missing_cols = Vec::new();

    for (row, line) in INPUT.lines().enumerate() {
        let mut found = false;

        if row == 0 {
            missing_cols = vec![true; line.len()];
        }

        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                found = true;

                if larger_expansion {
                    galaxies.push(Coord { x: col, y: row + (missing_rows * 999_999) });
                } else {
                    galaxies.push(Coord { x: col, y: row + missing_rows });
                }

                missing_cols[col] = false;
            }
        }

        if !found {
            missing_rows += 1;
        }
    }

    for coord in galaxies.iter_mut() {
        let missing = &missing_cols[0..coord.x].iter().fold(0_usize, |acc, e| {
            if *e {
                acc + 1
            } else {
                acc
            }
        });

        if larger_expansion {
            coord.x += missing * 999_999
        } else {
            coord.x += missing
        }
    }

    galaxies
}

pub fn solve1() -> usize {
    let mut galaxies = read_galaxies(false);
    let mut rc = 0;

    while let Some(curr) = galaxies.pop() {
        for g in galaxies.iter() {
            rc += curr.x.abs_diff(g.x) + curr.y.abs_diff(g.y);
        }
    }

    rc
}

pub fn solve2() -> usize {
    let mut galaxies = read_galaxies(true);
    let mut rc = 0;

    while let Some(curr) = galaxies.pop() {
        for g in galaxies.iter() {
            rc += curr.x.abs_diff(g.x) + curr.y.abs_diff(g.y);
        }
    }

    rc
}
