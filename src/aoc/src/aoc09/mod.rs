use crate::util::{read_i64, skip_whitespace};

static INPUT: &str = include_str!("input");

fn find_next(input: &mut Vec<i64>, reverse: bool) -> i64 {
    let mut last = Vec::with_capacity(input.len());

    if reverse {
        last.push(*input.first().unwrap());
    } else {
        last.push(*input.last().unwrap());
    }

    while !input.iter().fold(true, |b, v| b && *v == 0) {
        let mut row: Vec<_> = input.iter().zip(input.iter().skip(1)).map(|(a, b)| *b - *a).collect();

        if reverse {
            last.push(*row.first().unwrap());
        } else {
            last.push(*row.last().unwrap());
        }

        input.clear();
        input.append(&mut row);
    }

    if reverse {
        let mut prev = 0;

        for l in last.iter().rev() {
            prev = *l - prev;
        }

        prev
    } else {
        last.iter().sum()
    }
}

pub fn solve1() -> i64 {
    let mut problem = Vec::with_capacity(100);
    let mut rc = 0;

    for line in INPUT.lines() {
        let mut chars = line.chars().peekable();

        while chars.peek().is_some() {
            problem.push(read_i64(&mut chars));
            skip_whitespace(&mut chars);
        }

        let solve = find_next(&mut problem, false);
        rc += solve;
        problem.clear();
    }

    rc
}

pub fn solve2() -> i64 {
    let mut problem = Vec::with_capacity(100);
    let mut rc = 0;

    for line in INPUT.lines() {
        let mut chars = line.chars().peekable();

        while chars.peek().is_some() {
            problem.push(read_i64(&mut chars));
            skip_whitespace(&mut chars);
        }

        let solve = find_next(&mut problem, true);
        rc += solve;
        problem.clear();
    }

    rc
}
