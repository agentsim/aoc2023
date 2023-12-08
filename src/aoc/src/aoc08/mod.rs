use std::collections::HashMap;
use num::Integer;
use crate::util::{read_until, skip_until, skip_whitespace};

static INPUT: &str = include_str!("input");

pub fn solve1() -> u32 {
    let mut lines = INPUT.lines();
    let instructions: Vec<_> = lines.next().unwrap().chars().collect();
    let mut map = HashMap::with_capacity(1000);
    let mut curr = "AAA".to_string();
    let mut rc = 0;

    lines.next();

    for line in lines {
        let mut chars = line.chars().peekable();
        let key = read_until(&mut chars, |c| c == ' ');

        skip_until(&mut chars, |c| c == '(');
        chars.next();
        let l = read_until(&mut chars, |c| c == ',');
        chars.next();
        skip_whitespace(&mut chars);
        let r = read_until(&mut chars, |c| c == ')');

        map.insert(key, (l, r));
    }

    loop {
        for i in instructions.iter() {
            if let Some((l, r)) = map.get(&curr) {
                if *i == 'L' {
                    curr = l.clone();
                } else {
                    curr = r.clone();
                }
            }

            rc += 1;

            if curr == "ZZZ".to_string() {
                return rc;
            }
        }
    }
}

pub fn solve2() -> u64 {
    let mut lines = INPUT.lines();
    let instructions: Vec<_> = lines.next().unwrap().chars().collect();
    let mut map = HashMap::with_capacity(1000);
    let mut curr = Vec::with_capacity(100);
    let mut all_steps = Vec::with_capacity(100);

    lines.next();

    for line in lines {
        let mut chars = line.chars().peekable();
        let key = read_until(&mut chars, |c| c == ' ');

        skip_until(&mut chars, |c| c == '(');
        chars.next();
        let l = read_until(&mut chars, |c| c == ',');
        chars.next();
        skip_whitespace(&mut chars);
        let r = read_until(&mut chars, |c| c == ')');

        if key.ends_with('A') {
            curr.push(key.clone());
        }

        map.insert(key, (l, r));
    }

    for c in curr.iter_mut() {
        let mut steps = 0;

        'outer: loop {
            for i in instructions.iter() {
                steps += 1;

                if let Some((l, r)) = map.get(c) {
                    if *i == 'L' {
                        *c = l.clone();
                    } else if *i == 'R' {
                        *c = r.clone();
                    }

                    if c.ends_with('Z') {
                        break 'outer;
                    }
                }
            }
        }

        all_steps.push(steps);
    }

    let mut lcm: u64 = 0;

    for (i, v) in all_steps.iter().enumerate() {
        if i == 1 {
            lcm = all_steps[0].lcm(v);
        } else if i > 1 {
            lcm = lcm.lcm(v);
        }
    }

    lcm
}
