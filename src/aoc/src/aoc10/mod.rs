use crate::aoc10::CoordKind::{Outside, Path, Unknown};
use crate::aoc10::Dir::{East, North, South, West};

static INPUT: &str = include_str!("input");
//static INPUT: &str = include_str!("input_sample");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoordKind {
    Unknown,
    Path(char),
    Outside
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West
}

fn find_connection(grid: &Vec<Vec<char>>, origin: &Coord, ignore: Option<Dir>) -> (Option<Coord>, Dir) {
    if origin.y > 0 && ignore != Some(North) {
        let north = Coord { x: origin.x, y: origin.y - 1 };

        if let Some(next) = next(&grid[north.y][north.x], &north, origin) {
            return (Some(north), North)
        }
    }

    if origin.y < grid.len() - 1 && ignore != Some(South) {
        let south = Coord { x: origin.x, y: origin.y + 1 };

        if let Some(next) = next(&grid[south.y][south.x], &south, origin) {
            return (Some(south), South)
        }
    }

    if origin.x < grid[0].len() - 1 && ignore != Some(East) {
        let east = Coord { x: origin.x + 1, y: origin.y };

        if let Some(next) = next(&grid[east.y][east.x], &east, origin) {
            return (Some(east), East)
        }
    }

    if origin.x > 0 && ignore != Some(West) {
        let west = Coord { x: origin.x - 1, y: origin.y };

        if let Some(next) = next(&grid[west.y][west.x], &west, origin) {
            return (Some(west), West)
        }
    }

    (None, North)
}

fn next(ch: &char, coord: &Coord, prev: &Coord) -> Option<Coord> {
    if *ch == '|' {
        if coord.y != prev.y {
            Some(Coord { x: coord.x, y: coord.y + coord.y - prev.y })
        } else {
            None
        }
    } else if *ch == '-' {
        if coord.x != prev.x {
            Some(Coord { x: coord.x + coord.x - prev.x, y: coord.y })
        } else {
            None
        }
    } else if *ch == 'L' {
        if coord.y == prev.y {
            if coord.x == prev.x - 1 {
                Some(Coord { x: coord.x, y: coord.y - 1 })
            } else {
                None
            }
        } else {
            if coord.y == prev.y + 1 {
                Some(Coord { x: coord.x + 1, y: coord.y })
            } else {
                None
            }
        }
    } else if *ch == 'J' {
        if coord.y == prev.y {
            if coord.x == prev.x + 1 {
                Some(Coord { x: coord.x, y: coord.y - 1 })
            } else {
                None
            }
        } else {
            if coord.y == prev.y + 1 {
                Some(Coord { x: coord.x - 1, y: coord.y })
            } else {
                None
            }
        }
    } else if *ch == '7' {
        if coord.y == prev.y {
            if coord.x == prev.x + 1 {
                Some(Coord { x: coord.x, y: coord.y + 1 })
            } else {
                None
            }
        } else {
            if coord.y == prev.y - 1 {
                Some(Coord { x: coord.x - 1, y: coord.y })
            } else {
                None
            }
        }
    } else if *ch == 'F' {
        if coord.y == prev.y {
            if coord.x == prev.x - 1 {
                Some(Coord { x: coord.x, y: coord.y + 1 })
            } else {
                None
            }
        } else {
            if coord.y == prev.y - 1 {
                Some(Coord { x: coord.x + 1, y: coord.y })
            } else {
                None
            }
        }
    } else {
        None
    }
}

fn read_grid() -> (Vec<Vec<char>>, Coord) {
    let mut grid = Vec::with_capacity(1000);
    let mut origin = None;

    for (y, line) in INPUT.lines().enumerate() {
        let mut row = Vec::with_capacity(1000);

        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                origin = Some(Coord { x, y });
            }

            row.push(ch);
        }

        grid.push(row);
    }

    (grid, origin.unwrap())
}

fn init_classification(grid: &Vec<Vec<char>>, origin: &Coord) -> Vec<Vec<CoordKind>> {
    let mut classification_grid = Vec::with_capacity(grid.len());
    let (mut coord,  first_dir) = find_connection(&grid, &origin, None);
    let second_dir = find_connection(&grid, &origin, Some(first_dir)).1;
    let mut prev = *origin;
    let mut coord = coord.unwrap();

    for r in grid {
        classification_grid.push(vec![Unknown; r.len()]);
    }

    let piece = match (first_dir, second_dir) {
        (North, South) => '|',
        (East, West) => '-',
        (North, West) => 'J',
        (North, East) => 'L',
        (South, East) => 'F',
        (South, West) => '7',
        (_, _) => panic!()
    };

    classification_grid[origin.y][origin.x] = Path(piece);

    while coord != *origin {
        let next = next(&grid[coord.y][coord.x], &coord, &prev).unwrap();

        classification_grid[coord.y][coord.x] = Path(grid[coord.y][coord.x]);

        prev = coord;
        coord = next;
    }

    classification_grid
}

fn east(grid: &Vec<Vec<CoordKind>>, p: Coord) -> (Option<Coord>, Option<CoordKind>) {
    if p.x < grid[0].len() - 1 {
        (Some(Coord { x: p.x + 1, y: p.y }), Some(grid[p.y][p.x + 1]))
    } else {
        (None, None)
    }
}

fn west(grid: &Vec<Vec<CoordKind>>, p: Coord) -> (Option<Coord>, Option<CoordKind>) {
    if p.x > 0 {
        (Some(Coord { x: p.x - 1, y: p.y }), Some(grid[p.y][p.x - 1]))
    } else {
        (None, None)
    }
}

fn north(grid: &Vec<Vec<CoordKind>>, p: Coord) -> (Option<Coord>, Option<CoordKind>) {
    if p.y > 0 {
        (Some(Coord { x: p.x, y: p.y - 1 }), Some(grid[p.y - 1][p.x]))
    } else {
        (None, None)
    }
}

fn south(grid: &Vec<Vec<CoordKind>>, p: Coord) -> (Option<Coord>, Option<CoordKind>) {
    if p.y < grid.len() - 1 {
        (Some(Coord { x: p.x, y: p.y + 1 }), Some(grid[p.y + 1][p.x]))
    } else {
        (None, None)
    }
}

fn next_squeeze(grid: &Vec<Vec<CoordKind>>, p: Coord, dir: Dir, edge: Dir) -> Option<Coord> {
    //println!("next squeeze: {:?} {:?} {:?}", p, dir, edge);
    let (edge_coord, edge_kind) = {
        match edge {
            North => {
                north(grid, p)
            }
            South => {
                south(grid, p)
            }
            East => {
                east(grid, p)
            }
            West => {
                west(grid, p)
            }
        }
    };
    let (next_coord, next_kind) = {
        match dir {
            North => {
                north(grid, p)
            }
            South => {
                south(grid, p)
            }
            East => {
                east(grid, p)
            }
            West => {
                west(grid, p)
            }
        }
    };

    if let (Some(coord), Some(kind)) = (edge_coord, edge_kind) {
        match kind {
            Unknown => { Some(coord) }
            Path(ch) => {
                let b = match dir {
                    North | South => {
                        if edge == West {
                            ch == 'J' || ch == '7' || ch == '|'
                        } else {
                            ch == 'L' || ch == 'F' || ch == '|'
                        }
                    }
                    East | West => {
                        if edge == North {
                            ch == '-' || ch == 'L' || ch == 'J'
                        } else {
                            ch == '-' || ch == 'F' || ch == '7'
                        }
                    }
                };

                if b {
                    if let (Some(next), Some(kind)) = (next_coord, next_kind) {
                        match kind {
                            Unknown => Some(next),
                            Outside => None,
                            Path(_) => {
                                next_squeeze(grid, next, dir, edge)
                            }
                        }
                    } else {
                        panic!()
                    }
                } else {
                    None
                }
            }
            Outside => { None }
        }
    } else {
        None
    }
}

fn squeeze(grid: &Vec<Vec<CoordKind>>, p: Coord, dir: Dir) -> (Option<Coord>, Option<Coord>) {
    //println!("squeezing: {:?} {:?}", p, dir);

    match grid[p.y][p.x] {
        Unknown => { return (Some(p), None); }
        Path(ch) => {
            match dir {
                North | South => {
                    if ch == 'L' || ch == 'F' {
                        (next_squeeze(grid, p, dir, West), None)
                    } else if ch == '7' || ch == 'J' {
                        (next_squeeze(grid, p, dir, East), None)
                    } else if ch == '|' {
                        (next_squeeze(grid, p, dir, East), next_squeeze(grid, p, dir, West))
                    } else {
                        (None, None)
                    }
                }
                East | West => {
                    if ch == 'L' || ch == 'J' {
                        (next_squeeze(grid, p, dir, South), None)
                    } else if ch == '7' || ch == 'F' {
                        (next_squeeze(grid, p, dir, North), None)
                    } else if ch == '-' {
                        (next_squeeze(grid, p, dir, North), next_squeeze(grid, p, dir, South))
                    } else {
                        (None, None)
                    }
                }
            }
        }
        Outside => { (None, None) }
    }
}

fn paint(grid: &mut Vec<Vec<CoordKind>>, p: Coord) {
    let width = grid[0].len();

    grid[p.y][p.x] = Outside;

    // North
    if p.y > 0 {
        match grid[p.y - 1][p.x] {
            Path(_) => {
                let (l, r) = squeeze(grid, Coord { x: p.x, y: p.y - 1}, North);

                if let Some(l) = l {
                    paint(grid, l);
                }

                if let Some(r) = r {
                    paint(grid, r);
                }
            },
            Unknown => paint(grid, Coord { x: p.x, y: p.y - 1}),
            Outside => {}
        }
    }

    // South
    if p.y < grid.len() - 1 {
        match grid[p.y + 1][p.x] {
            Path(_) => {
                let (l, r) = squeeze(grid, Coord { x: p.x, y: p.y + 1}, South);

                if let Some(l) = l {
                    paint(grid, l);
                }

                if let Some(r) = r {
                    paint(grid, r);
                }
            },
            Unknown => paint(grid, Coord { x: p.x, y: p.y + 1}),
            Outside => {}
        }
    }

    // East
    if p.x < width - 1 {
        match grid[p.y][p.x + 1] {
            Path(_) => {
                let (l, r) = squeeze(grid, Coord { x: p.x + 1, y: p.y }, East);

                if let Some(l) = l {
                    paint(grid, l);
                }

                if let Some(r) = r {
                    paint(grid, r);
                }
            },
            Unknown => paint(grid, Coord { x: p.x + 1, y: p.y}),
            Outside => {}
        }
    }

    // West
    if p.x > 0 {
        match grid[p.y][p.x - 1] {
            Path(_) => {
                let (l, r) = squeeze(grid, Coord { x: p.x - 1, y: p.y }, West);

                if let Some(l) = l {
                    paint(grid, l);
                }

                if let Some(r) = r {
                    paint(grid, r);
                }
            },
            Unknown => paint(grid, Coord { x: p.x - 1, y: p.y}),
            Outside => {}
        }
    }
}

pub fn solve1() -> u32 {
    let (grid, origin) = read_grid();
    let mut coord = find_connection(&grid, &origin, None).0.unwrap();
    let mut prev = origin;
    let mut rc = 1;

    while coord != origin {
        let next = next(&grid[coord.y][coord.x], &coord, &prev).unwrap();

        prev = coord;
        coord = next;
        rc += 1;
    }

    (((rc as f32) + 0.5) / 2.).ceil() as u32
}

pub fn solve2() -> usize {
    let (grid, origin) = read_grid();
    let mut grid = init_classification(&grid, &origin);

    //print_grid(&grid);

    for i in 0..grid.len() {
        if grid[i][0] == CoordKind::Unknown {
            paint(&mut grid, Coord { x: 0, y: i});
        }
    }

    //println!("\n\n\n");
    //print_grid(&grid);

    let mut rc = 0;

    for r in grid {
        rc += r.iter().filter(|k| **k == Unknown).count()
    }

    rc
}

fn print_grid(grid: &Vec<Vec<CoordKind>>) {
    let border = vec!['#'; grid[0].len()];

    for c in &border {
        print!("{}", *c);
    }

    for r in grid {
        print!("\n");

        for c in r {
            match c {
                Unknown => print!("."),
                Path(ch) => print!("{}", ch),
                Outside => print!(" ")
            }
        }
    }

    print!("\n");

    for c in &border {
        print!("{}", *c);
    }

    print!("\n");
}