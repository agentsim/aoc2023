static INPUT: &str = include_str!("input");

struct Coord {
    x: usize,
    y: usize
}

fn read_galaxies(factor: usize) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut galaxy_x = Vec::with_capacity(500);
    let mut y_coord_count = Vec::with_capacity(500);
    let mut missing_rows = 0;

    for (row, line) in INPUT.lines().enumerate() {
        let mut count = 0;

        if row == 0 {
            galaxy_x = vec![0_usize; line.len()];
        }

        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                count += 1;
                galaxy_x[col] += 1;
            }
        }

        if count == 0 {
            missing_rows += 1;
        } else {
            y_coord_count.push((row + missing_rows * factor, count));
        }
    }

    let mut x_coord_count = Vec::with_capacity(galaxy_x.len());

    let mut missing = 0;

    for (idx, count) in galaxy_x.iter().enumerate() {
        if *count == 0 {
            missing += 1;
        } else {
            x_coord_count.push((idx + missing * factor, *count));
        }
    }

    missing = 0;

    (x_coord_count, y_coord_count)
}

fn solve(larger_expansion: bool) -> usize {
    let factor = if larger_expansion {
        999_999
    } else {
        1
    };
    let (mut x, mut y) = read_galaxies(factor);
    let mut rc = 0;

    while let Some((idx, count)) = x.pop() {
        for (x, c) in x.iter() {
            rc += idx.abs_diff(*x) * (count * c);
        }
    }

    while let Some((idx, count)) = y.pop() {
        for (y, c) in y.iter() {
            rc += idx.abs_diff(*y) * (count * c);
        }
    }

    rc
}

pub fn solve1() -> usize {
    solve(false)
}

pub fn solve2() -> usize {
    solve(true)
}
