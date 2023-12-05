use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::iter::Peekable;
use std::str::{Chars, Lines};
use crate::util::{read_u64, read_until, skip_until, skip_whitespace};

static INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Range {
    src: u64,
    dst: u64,
    len: u64
}

impl Range {
    pub fn new(src: u64, dst: u64, len: u64) -> Range {
        Range { src, dst, len }
    }

    pub fn end(&self) -> u64 {
        self.src + self.len
    }
}

impl PartialEq<Self> for Range {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(&other).unwrap() == Ordering::Equal
    }
}

impl PartialOrd<Self> for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let begin_1 = self.src;
        let end_1 = self.src + self.len;
        let begin_2 = other.src;
        let end_2 = other.src + other.len;

        if end_1 <= begin_2 {
            Some(Ordering::Less)
        } else if end_2 <= begin_1 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for Range {}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

struct Mapping {
    src: String,
    dst: String,
    mapping: BTreeSet<Range>
}

impl Mapping {
    pub fn new(src: String, dst: String) -> Mapping {
        Mapping {
            src,
            dst,
            mapping: BTreeSet::new()
        }
    }

    pub fn add(&mut self, range: Range) {
        if !self.mapping.insert(range) {
            panic!();
        }
    }

    pub fn lookup(&self, v: u64) -> u64 {
        let r = Range::new(v, v, 0);

        if let Some(mapping) = self.mapping.get(&r) {
            mapping.dst + v - mapping.src
        } else {
            v
        }
    }

    pub fn lookup_range(&self, r: (u64, u64)) -> Vec<(u64, u64)> {
        let mut rc = Vec::with_capacity(100);
        let (mut begin, mut end) = (r.0, r.0 + r.1);

        while let Some(mapping) = self.mapping.get(&Range::new(begin, begin, 1)) {
            let map_end = mapping.end();
            let map_offset = begin - mapping.src;

            if map_end >= end {
                rc.push((mapping.dst + map_offset, end - begin));
                return rc;
            } else {
                rc.push((mapping.dst + map_offset, mapping.len - map_offset));
                begin += mapping.len - map_offset;
            }
        }

        rc.push((begin, end - begin));
        rc
    }
}

fn read_seeds(line: &str) -> Vec<u64> {
    let mut chars = line.chars().peekable();
    let mut rc = Vec::with_capacity(100);

    skip_until(&mut chars, |ch| ch == ':');
    chars.next();
    skip_whitespace(&mut chars);

    while chars.peek().is_some() {
        rc.push(read_u64(&mut chars));
        skip_whitespace(&mut chars);
    }

    rc
}

fn read_seed_pairs(line: &str) -> Vec<(u64, u64)> {
    let mut chars = line.chars().peekable();
    let mut rc = Vec::with_capacity(1000);

    skip_until(&mut chars, |ch| ch == ':');
    chars.next();
    skip_whitespace(&mut chars);

    while chars.peek().is_some() {
        let src = read_u64(&mut chars);

        skip_whitespace(&mut chars);

        let len = read_u64(&mut chars);

        skip_whitespace(&mut chars);
        rc.push((src, len));
    }

    rc
}

fn read_map(chars: &mut Peekable<Chars>) -> (u64, u64, u64) {
    let dst = read_u64(chars);

    skip_whitespace(chars);
    let src = read_u64(chars);

    skip_whitespace(chars);
    let len = read_u64(chars);

    (src, dst, len)
}

fn read_header(chars: &mut Peekable<Chars>) -> (String, String) {
    let src = read_until(chars, |ch| ch == '-');

    chars.next();
    skip_until(chars, |ch| ch == '-');
    chars.next();

    (src, read_until(chars, |ch| ch == ' '))
}

fn read_full_mapping(lines: &mut Lines) -> Option<Mapping> {
    if let Some(line) = lines.next() {
        let mut chars = line.chars().peekable();
        let (src, dst) = read_header(&mut chars);
        let mut rc = Mapping::new(src, dst);

        while let Some(line) = lines.next() {
            let mut chars = line.chars().peekable();

            if chars.peek().is_some() {
                let (src, dst, len) = read_map(&mut chars);

                rc.add(Range::new(src, dst, len));
            } else {
                break
            }
        }

        Some(rc)
    } else {
        None
    }
}

pub fn solve1() -> u64 {
    let mut lines = INPUT.lines();
    let mut lookups = read_seeds(lines.next().unwrap());
    let mut mappings = HashMap::new();
    let mut src = String::from("seed");

    lines.next();

    while let Some(mapping) = read_full_mapping(&mut lines) {
        mappings.insert(mapping.src.clone(), mapping);
    }

    while let Some(mapping) = mappings.get(&src) {
        for v in lookups.iter_mut() {
            *v = mapping.lookup(*v);
        }

        src = mapping.dst.clone();
    }

    *lookups.iter().min().unwrap()
}

pub fn solve2() -> u64 {
    let mut lines = INPUT.lines();
    let mut lookups = read_seed_pairs(lines.next().unwrap());
    let mut mappings = HashMap::new();
    let mut src = String::from("seed");

    lines.next();

    while let Some(mapping) = read_full_mapping(&mut lines) {
        mappings.insert(mapping.src.clone(), mapping);
    }

    while let Some(mapping) = mappings.get(&src) {
        let mut revised_lookups = Vec::with_capacity(lookups.len() * 2);

        for v in lookups.iter_mut() {
            revised_lookups.append(&mut mapping.lookup_range(*v));
        }

        src = mapping.dst.clone();
        lookups = revised_lookups;
    }

    lookups.iter().map(|(b, _)| *b).min().unwrap()
}
