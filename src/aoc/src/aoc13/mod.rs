static INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Axis {
    val: usize,
    idx: usize
}

fn read_grid() -> Vec<((Vec<usize>, usize), (Vec<usize>, usize))> {
    let mut rc = Vec::with_capacity(100);
    let mut lines = INPUT.lines().peekable();

    while lines.peek().is_some() {
        let mut x = Vec::with_capacity(100);
        let mut y = Vec::with_capacity(100);
        let mut row = 0_usize;

        while let Some(line) = lines.next() {
            let mut row_val = 0_usize;

            if line.len() == 0 {
                break;
            }

            for (c, ch) in line.chars().enumerate() {
                if row == 0 {
                    y.push(0_usize);
                }

                if ch == '#' {
                    row_val += (1 << c);
                    y[c] += (1 << row);
                }
            }

            x.push(row_val);
            row += 1;
        }

        let x_len = x.len();
        let y_len = y.len();

        rc.push(((x, y_len), (y, x_len)));
    }

    rc
}

fn reflection_point(row: &[usize], ignore: Option<usize>) -> Option<usize> {
    for (idx, (l, r)) in row.iter().zip(row.iter().skip(1)).enumerate() {
        if Some(idx) != ignore {
            if l == r {
                let mut ok = true;
                let _ = row.iter().rev().skip(row.len() - idx - 1).zip(row.iter().skip(idx + 1)).filter(|(l, r)| {
                    ok = ok && l == r;
                    ok
                }).count();

                if ok {
                    return Some(idx);
                }
            }
        }
    }

    None
}

fn reflection_point_smudged(row: &mut Vec<usize>, width: usize, ignore: Option<usize>) -> Option<usize> {
    for r in 0..row.len() {
        for i in 0..width {
            let old = row[r];

            row[r] = old ^ (1 << i);

            if let Some(result) = reflection_point(row, ignore) {
                row[r] = old;
                return Some(result);
            }

            row[r] = old;
        }
    }

    None
}

fn solve(smudged: bool) -> usize {
    let mut grids = read_grid();
    let mut rc = 0;

    for ((row, row_len), (col, col_len)) in grids.iter_mut() {
        if smudged {
            let row_result = reflection_point(row, None);
            let col_result = reflection_point(col, None);

            if let Some(idx) = reflection_point_smudged(row, *row_len, row_result) {
                rc += (idx + 1) * 100;
            } else if let Some(idx) = reflection_point_smudged(col, *col_len, col_result) {
                rc += idx + 1;
            } else {
            }
        } else {
            if let Some(idx) = reflection_point(row, None) {
                rc += (idx + 1) * 100;
            } else if let Some(idx) = reflection_point(col, None) {
                rc += idx + 1;
            }
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
