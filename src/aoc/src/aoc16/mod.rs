static INPUT: &[u8] = include_bytes!("input");
//static INPUT: &[u8] = include_bytes!("input_sample");

const NORTH: u8 = 1;
const SOUTH: u8 = 2;
const EAST: u8 = 4;
const WEST: u8 = 8;

fn read_grid() -> Vec<Vec<(u8, u8)>> {
    let mut rc = Vec::with_capacity(150);
    let tmp = Vec::with_capacity(150);

    rc.push(tmp);

    let mut tmp = rc.last_mut().unwrap();
    let mut iter = INPUT.iter();

    while let Some(b) = iter.next() {
        if *b == b'\n' {
            rc.push(Vec::with_capacity(150));
            tmp = rc.last_mut().unwrap();
        } else {
            tmp.push((*b, 0));
        }
    }

    if rc.last().unwrap().is_empty() {
        rc.remove(rc.len() - 1);
    }

    rc
}

fn count_energized(grid: &mut Vec<Vec<(u8, u8)>>, init: ((usize, usize), u8)) -> usize {
    let mut beams = Vec::with_capacity(50);

    beams.push(init);

    while let Some(((mut row, mut col), mut dir)) = beams.pop() {
        loop {
            grid[row][col].1 |= dir;

            match grid[row][col].0 {
                b'/' => {
                    if dir == EAST {
                        dir = NORTH
                    } else if dir == WEST {
                        dir = SOUTH
                    } else if dir == SOUTH {
                        dir = WEST
                    } else {
                        dir = EAST
                    }
                },
                b'\\' => {
                    if dir == EAST {
                        dir = SOUTH
                    } else if dir == WEST {
                        dir = NORTH
                    } else if dir == SOUTH {
                        dir = EAST
                    } else {
                        dir = WEST
                    }
                },
                b'-' => {
                    if dir == NORTH || dir == SOUTH {
                        beams.push(((row, col), EAST));
                        dir = WEST;
                    }
                },
                b'|' => {
                    if dir == EAST || dir == WEST {
                        beams.push(((row, col), NORTH));
                        dir = SOUTH;
                    }
                },
                _ => {}
            }

            match dir {
                EAST => col += 1,
                WEST => {
                    if col == 0 {
                        break;
                    } else {
                        col -= 1
                    }
                },
                NORTH => {
                    if row == 0 {
                        break;
                    } else {
                        row -= 1
                    }
                },
                SOUTH => row += 1,
                _ => {}
            };

            if let Some((_, d)) = grid.get(row).and_then(|v| v.get(col)) {
                if *d & dir != 0 {
                    break;
                }
            } else {
                break
            }
        }
    }

    //print_grid(&grid);
    grid.iter().map(|v| v.iter().map(|(_, d)| if *d != 0 { 1 } else { 0} ).sum::<usize>()).sum()

}

pub fn solve1() -> usize {
    let mut grid = read_grid();

    count_energized(&mut grid, ((0, 0), EAST))
}

pub fn solve2() -> usize {
    let mut grid = read_grid();
    let mut max = 0;
    let width = grid[0].len();
    let height = grid.len();

    for i in 0..width {
        let e = count_energized(&mut grid, ((0, i), SOUTH));

        max = max.max(e);

        for v in grid.iter_mut() {
            for (_, d) in v.iter_mut() {
                *d = 0;
            }
        }

        let e = count_energized(&mut grid, ((height - 1, i), NORTH));

        max = max.max(e);

        for v in grid.iter_mut() {
            for (_, d) in v.iter_mut() {
                *d = 0;
            }
        }
    }

    for i in 0..height {
        let e = count_energized(&mut grid, ((i, 0), EAST));

        max = max.max(e);

        for v in grid.iter_mut() {
            for (_, d) in v.iter_mut() {
                *d = 0;
            }
        }

        let e = count_energized(&mut grid, ((i, width - 1), WEST));

        max = max.max(e);

        for v in grid.iter_mut() {
            for (_, d) in v.iter_mut() {
                *d = 0;
            }
        }

    }

    max
}

fn print_grid(grid: &Vec<Vec<(u8, u8)>>) {
    for row in grid {
        for (_, d) in row {
            if *d == 0 {
                print!(".")
            } else {
                print!("#")
            }
        }

        print!("\n")
    }
}