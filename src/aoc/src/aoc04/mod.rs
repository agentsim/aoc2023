use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::str::Chars;
use crate::util::{read_number, skip_until, skip_whitespace};

static INPUT: &str = include_str!("input");

fn read_numbers(chars: &mut Peekable<Chars>, numbers: &mut HashSet<u32>) {
    skip_until(chars, |ch| { ch == ':'});
    chars.next();

    while let Some(ch) = chars.peek() {
        if *ch == '|' {
            break
        }

        skip_whitespace(chars);
        numbers.insert(read_number(chars));
        skip_whitespace(chars);
    }
}

fn count_winners(chars: &mut Peekable<Chars>, numbers: &HashSet<u32>) -> usize {
    let mut rc = 0;

    while let Some(ch) = chars.peek() {
        skip_whitespace(chars);

        if numbers.contains(&read_number(chars)) {
            rc += 1;
        }

        skip_whitespace(chars);
    }

    rc
}

pub fn solve1() -> u32 {
    let mut rc = 0;
    let mut numbers = HashSet::with_capacity(100);

    for line in INPUT.lines() {
        let mut chars = line.chars().peekable();

        read_numbers(&mut chars, &mut numbers);
        chars.next();

        let winners = count_winners(&mut chars, &numbers);

        if winners > 0 {
            rc += 1 << (winners - 1);
        }

        numbers.clear();
    }

    rc
}

pub fn solve2() -> u32 {
    let mut copies = HashMap::new();
    let mut numbers = HashSet::with_capacity(100);

    for (idx, line) in INPUT.lines().enumerate() {
        let mut chars = line.chars().peekable();

        read_numbers(&mut chars, &mut numbers);
        chars.next();

        let curr_copies = if let Some(v) = copies.get_mut(&idx) {
            *v += 1;
            *v
        } else {
            copies.insert(idx, 1);
            1
        };
        let mut winners = count_winners(&mut chars, &numbers);

        while winners > 0 {
            if let Some(v) = copies.get_mut(&(idx + winners)) {
                *v += curr_copies;
            } else {
                copies.insert(idx + winners, curr_copies);
            };

            winners -= 1
        }

        numbers.clear();
    }

    copies.iter().map(|(k, v)| *v).sum()
}
