use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use crate::aoc14::Entry::{Empty, Rock, Round};

static INPUT: &str = include_str!("input");
//static INPUT: &str = include_str!("input_sample");

#[derive(Copy, Clone)]
struct Segment {
    start: usize,
    rocks: usize
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Entry {
    Round,
    Rock,
    Empty
}

fn read_grid() -> Vec<Vec<Entry>> {
    let mut rc = Vec::with_capacity(100);

    for line in INPUT.lines() {
        let mut r = Vec::with_capacity(100);

        for ch in line.chars() {
            if ch == 'O' {
                r.push(Round);
            } else if ch == '#' {
                r.push(Rock);
            } else if ch == '.' {
                r.push(Empty);
            }
        }

        rc.push(r);
    }

    rc
}

fn hash<'a, I>(direction: Direction, values: I) -> u64
where I: IntoIterator<Item = &'a Entry> {
    let mut hasher = DefaultHasher::new();

    direction.hash(&mut hasher);

    for v in values {
        v.hash(&mut hasher);
    }

    hasher.finish()
}

fn tilt_north(grid: &mut Vec<Vec<Entry>>) {
    let width  = grid[0].len();

    for col in 0..width {
        let mut next_slot: Option<usize> = None;

            for row in 0..grid.len() {
                match grid[row][col] {
                    Round => {
                        if let Some(ref mut next_slot) = next_slot {
                            grid[*next_slot][col] = Round;
                            grid[row][col] = Empty;
                            *next_slot += 1;
                        }
                    }
                    Rock => {
                        next_slot = None;
                    }
                    Empty => {
                        if next_slot.is_none() {
                            next_slot = Some(row);
                        }
                    }
                }
            }
    }
}

fn tilt_south(grid: &mut Vec<Vec<Entry>>) {
    let width  = grid[0].len();

    for col in 0..width {
        let mut next_slot: Option<usize> = None;

            for row in (0..grid.len()).rev() {
                match grid[row][col] {
                    Round => {
                        if let Some(ref mut next_slot) = next_slot {
                            grid[*next_slot][col] = Round;
                            grid[row][col] = Empty;
                            *next_slot -= 1;
                        }
                    }
                    Rock => {
                        next_slot = None;
                    }
                    Empty => {
                        if next_slot.is_none() {
                            next_slot = Some(row);
                        }
                    }
                }
            }
    }
}

fn tilt_west(grid: &mut Vec<Vec<Entry>>) {
    let width  = grid[0].len();
    let height = grid.len();

    for row in 0..height {
        let mut next_slot: Option<usize> = None;

            for col in 0..width {
                match grid[row][col] {
                    Round => {
                        if let Some(ref mut next_slot) = next_slot {
                            grid[row][*next_slot] = Round;
                            grid[row][col] = Empty;
                            *next_slot += 1;
                        }
                    }
                    Rock => {
                        next_slot = None;
                    }
                    Empty => {
                        if next_slot.is_none() {
                            next_slot = Some(col);
                        }
                    }
                }
            }
    }
}

fn tilt_east(grid: &mut Vec<Vec<Entry>>) {
    let width  = grid[0].len();
    let height = grid.len();

    for row in 0..height {
        let mut next_slot: Option<usize> = None;
            for col in (0..width).rev() {
                match grid[row][col] {
                    Round => {
                        if let Some(ref mut next_slot) = next_slot {
                            grid[row][*next_slot] = Round;
                            grid[row][col] = Empty;
                            *next_slot -= 1;
                        }
                    }
                    Rock => {
                        next_slot = None;
                    }
                    Empty => {
                        if next_slot.is_none() {
                            next_slot = Some(col);
                        }
                    }
                }
            }
    }
}

pub fn solve1() -> usize {
    let mut column_segments = Vec::with_capacity(100);
    let mut tmp_segments: &mut [Option<Segment>] = &mut [None; 100];
    let mut row = 0;
    let mut lines = INPUT.lines();

    while let Some(line) = lines.next() {
        for (col, ch) in line.chars().enumerate() {
            if row == 0 {
                column_segments.push(Vec::with_capacity(50));
            }

            if ch == 'O' {
                if let Some(ref mut segment) = tmp_segments[col] {
                    segment.rocks += 1;
                } else {
                    tmp_segments[col] = Some(Segment { start: row, rocks: 1 });
                }
            } else if ch == '#' {
                if let Some(segment) = tmp_segments[col].take() {
                    column_segments[col].push(segment);
                }
            } else if ch == '.' {
                if tmp_segments[col].is_none() {
                    tmp_segments[col] = Some(Segment { start: row, rocks: 0 });
                }
            }
        }

        row += 1;
    }

    for (idx, segment) in tmp_segments.into_iter().enumerate() {
        if let Some(segment) = segment.take() {
            column_segments[idx].push(segment);
        }
    }

    column_segments.iter().map(|v| {
        v.iter().map(|s| {
            if s.rocks > 0 {
                let t = row - s.start;
                let n = (s.rocks - 1) * s.rocks / 2;

                t * s.rocks - n
            } else {
                0
            }
        }).sum::<usize>()
    }).sum()
}

fn spin(grid: &mut Vec<Vec<Entry>>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

pub fn solve2() -> usize {
    let mut grid = read_grid();
    let mut hasher = DefaultHasher::new();
    let mut hashes = HashMap::with_capacity(1000);
    let mut i = 0;
    let mut loop_begin = 0;

    grid.hash(&mut hasher);

    hashes.insert(hasher.finish(), 0);

    loop {
        spin(&mut grid);
        i += 1;
        hasher = DefaultHasher::new();
        grid.hash(&mut hasher);

        if let Some(found) = hashes.insert(hasher.finish(), i) {
            loop_begin = found;
            break;
        }
    }

    let loop_length = i - loop_begin;
    let remainder = (1000000000 - loop_begin) % (loop_length);

    for _ in 0..remainder {
        spin(&mut grid)
    }

    let height = grid.len();
    let mut rc = 0;

    for (row, col) in grid.iter().enumerate() {
        for entry in col {
            if *entry == Round {
                rc += height - row
            }
        }
    }

    rc
}

fn print_grid(grid: &Vec<Vec<Entry>>) {
    for r in grid {
        for c in r {
            match c {
                Round => print!("O"),
                Rock => print!("#"),
                Empty => print!(".")
            }
        }
        print!("\n");
    }
}