use crate::util::{read_u64, skip_until, skip_whitespace};

static INPUT: &str = include_str!("input");
//static INPUT: &str = include_str!("input_sample");

struct Record {
    length: usize,
    known_working: usize,
    known_broken: usize,
    broken_pattern: Vec<usize>
}

struct WideRecord {
    length: usize,
    known_working: u128,
    known_broken: u128,
    broken_pattern: Vec<usize>
}

fn read_record(line: &str) -> Record {
    let mut length = 0;
    let mut known_working = 0_usize;
    let mut known_broken = 0_usize;
    let mut chars = line.chars().peekable();
    let mut pattern = Vec::with_capacity(10);
    let mut i = 0;

    while let Some(ch) = chars.next() {
        if ch == '.' {
            known_working = known_working | (1 << i);
        } else if ch == '#' {
            known_broken = known_broken | (1 << i);
        } else if ch == ' ' {
            length = i;
            break
        }

        i += 1;
    }

    loop {
        pattern.push(read_u64(&mut chars) as usize);

        if chars.peek().is_none() {
            break
        }
    }

    pattern.reverse();

    Record {
        length, known_working, known_broken, broken_pattern: pattern
    }
}

fn read_wide_record(line: &str) -> WideRecord {
    let mut length = 0;
    let mut known_working = 0_u128;
    let mut known_broken = 0_u128;
    let mut pattern = Vec::with_capacity(10);
    let mut i = 0;

    for _ in 0..5 {
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '.' {
                known_working = known_working | (1 << i);
            } else if ch == '#' {
                known_broken = known_broken | (1 << i);
            } else if ch == ' ' {
                length = i;
                break
            }

            i += 1;
        }

        loop {
            pattern.push(read_u64(&mut chars) as usize);

            if chars.peek().is_none() {
                break
            }
        }

        i += 1;
    }


    pattern.reverse();

    WideRecord {
        length, known_working, known_broken, broken_pattern: pattern
    }
}

fn count_permutations_old(record: &Record, len: usize, mut pos: usize, remaining: &[usize], value: usize) -> usize {
    let mut rc = 0;
    let remaining_len = remaining.iter().sum::<usize>() + remaining.len();

    while pos + len + remaining_len <= record.length {
        let mut v = value;
        let mask = if remaining.is_empty() {
            0
        } else {
            (1 << (record.length - pos - len)) - 1
        };

        v = v | ((1 << len) - 1) << (record.length - pos - len);

        if v & record.known_working == 0 && (v | mask) & record.known_broken == record.known_broken {
            if remaining.is_empty() {
                rc += 1;
            } else {
                rc += count_permutations_old(record, remaining[0], pos + len + 1, &remaining[1..], v);
            }
        }

        pos += 1;
    }

    rc
}

fn count_permutations(record: &Record, len: usize, mut pos: usize, remaining: &[usize]) -> Vec<usize> {
    let mut rc = Vec::with_capacity(100);
    let remaining_len = remaining.iter().sum::<usize>() + remaining.len();
    let mut offset = 0;
    let mut descendent_permutations = Vec::new();
    let first = pos == 0;
    let start = pos;

    while pos + len + remaining_len <= record.length {
        let v = ((1 << len) - 1) << (record.length - pos - len);
        let parent_mask = if first {
            usize::MAX << (record.length - start)
        } else {
            usize::MAX << (record.length - pos)
        };
        let descendant_mask = if remaining.is_empty() {
            0
        } else {
            (1 << (record.length - pos - len - 1)) - 1
        };
        let mask = parent_mask | descendant_mask;

        if offset == 0 && !remaining.is_empty() {
            descendent_permutations = count_permutations(record, remaining[0], pos + len + 1, &remaining[1..]);
        }

        if v & record.known_working == 0 && (v | mask) & record.known_broken == record.known_broken {
            if remaining.is_empty() {
                rc.push(1);
            } else {
                let mut sum = 0;

                for (i, c) in descendent_permutations[offset..].iter().enumerate() {
                    let mask = parent_mask | (descendant_mask >> i);

                    if (v | mask) & record.known_broken == record.known_broken {
                        sum += c;
                    }
                }

                rc.push(sum);
            }
        } else {
            rc.push(0);
        }

        pos += 1;
        offset += 1;
    }

    rc
}

fn count_permutations_wide(record: &WideRecord, len: usize, mut pos: usize, remaining: &[usize]) -> Vec<usize> {
    let mut rc = Vec::with_capacity(100);
    let remaining_len = remaining.iter().sum::<usize>() + remaining.len();
    let mut offset = 0;
    let mut descendent_permutations = Vec::new();
    let first = pos == 0;
    let start = pos;

    while pos + len + remaining_len <= record.length {
        let v = ((1 << len) - 1) << (record.length - pos - len);
        let parent_mask = if first {
            u128::MAX << (record.length - start)
        } else {
            u128::MAX << (record.length - pos)
        };
        let descendant_mask = if remaining.is_empty() {
            0
        } else {
            (1 << (record.length - pos - len - 1)) - 1
        };
        let mask = parent_mask | descendant_mask;

        if offset == 0 && !remaining.is_empty() {
            descendent_permutations = count_permutations_wide(record, remaining[0], pos + len + 1, &remaining[1..]);
        }

        if v & record.known_working == 0 && (v | mask) & record.known_broken == record.known_broken {
            if remaining.is_empty() {
                rc.push(1);
            } else {
                let mut sum = 0;

                for (i, c) in descendent_permutations[offset..].iter().enumerate() {
                    let mask = parent_mask | (descendant_mask >> i);

                    if (v | mask) & record.known_broken == record.known_broken {
                        sum += c;
                    }
                }

                rc.push(sum);
            }
        } else {
            rc.push(0);
        }

        pos += 1;
        offset += 1;
    }

    rc
}

pub fn solve1() -> usize {
    let mut rc = 0;
    let mut len = 0;

    for line in INPUT.lines() {
        let record = read_record(line);

        len = len.max(record.length);
        let c: usize =  count_permutations(&record, record.broken_pattern[0], 0, &record.broken_pattern[1..]).iter().sum();

        rc += c;
    }

    rc
}

pub fn solve2() -> usize {
    let mut rc = 0;
    let mut len = 0;

    for line in INPUT.lines() {
        let record = read_wide_record(line);

        len = len.max(record.length);
        let c: usize =  count_permutations_wide(&record, record.broken_pattern[0], 0, &record.broken_pattern[1..]).iter().sum();

        rc += c;
    }

    rc
}
