use std::str::Chars;

static INPUT: &str = include_str!("input");

struct Adjacency<'a> {
    chars: Option<Chars<'a>>,
    prev: Option<char>,
    curr: Option<char>,
    next: Option<char>
}

impl<'a> Adjacency<'a> {
    pub fn new(data: Option<&str>) -> Adjacency {
        if let Some(s) = data {
            let mut chars = s.chars();
            let next = chars.next();

            Adjacency {
                chars: Some(chars),
                prev: None,
                curr: None,
                next
            }
        } else {
            Adjacency {
                chars: None,
                prev: None,
                curr: None,
                next: None
            }
        }
    }
}

impl<'a> Iterator for Adjacency<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut chars) = self.chars {
            self.prev = self.curr;
            self.curr = self.next;
            self.next = chars.next();

            if let Some(ref ch) = self.curr {
                let prev = self.prev.unwrap_or('.');
                let next = self.next.unwrap_or('.');
                let is_non_symbol = (*ch == '.' || ch.is_digit(10)) &&
                    (prev == '.' || prev.is_digit(10)) &&
                    (next == '.' || next.is_digit(10));

                Some(!is_non_symbol)
            } else {
                None
            }
        } else {
            Some(false)
        }
    }
}

fn read_num_left(chars: &[char], mut idx: usize) -> Option<u32> {
    let mut mul = 1;
    let mut rc = None;

    idx -= 1;

    while idx >= 0 {
        if let Some(v) = chars[idx].to_digit(10) {
            if let Some(ref mut rc) = rc {
                *rc = *rc + v * mul;
            } else {
                rc = Some(v);
            }

            mul *= 10;

            if idx == 0 {
                break
            } else {
                idx -= 1;
            }
        } else {
            break
        }
    }

    rc
}

fn read_num_right(chars: &[char], mut idx: usize) -> Option<u32> {
    let mut rc = None;

    idx += 1;

    while idx < chars.len() {
        if let Some(v) = chars[idx].to_digit(10) {
            if let Some(ref mut rc) = rc {
                *rc = *rc * 10 + v;
            } else {
                rc = Some(v);
            }

            idx += 1;
        } else {
            break
        }
    }

    rc
}

fn read_adj_numbers(chars: &[char], mut idx: usize) -> (u32, u32) {
    if idx < chars.len() {
        if chars[idx].is_digit(10) {
            idx = idx - 1;

            while idx >= 0 && chars[idx].is_digit(10) {
                idx -= 1;
            }

            if let Some(num) = read_num_right(chars, idx) {
                (1, num)
            } else {
                (0, 0)
            }
        } else {
            let mut count = 0;
            let mut ratio = 0;

            if let Some(num) = read_num_left(chars, idx) {
                count = 1;
                ratio = num;
            }

            if let Some(num) = read_num_right(chars, idx) {
                count += 1;

                if count == 2 {
                    ratio *= num;
                } else {
                    ratio = num;
                }
            }

            if count == 0 {
                (0, 0)
            } else {
                (count, ratio)
            }
        }
    } else {
        (0, 0)
    }
}

fn sum_parts(prev: Option<&str>, curr: &str, next: Option<&str>) -> u32 {
    let mut adj_prev = Adjacency::new(prev);
    let mut adj_next = Adjacency::new(next);
    let mut chars = curr.chars().peekable();
    let mut ch_prev = '.';
    let mut ch_curr = '.';
    let mut ch_next = chars.peek().map(|c| *c).unwrap_or('.');
    let mut curr_part_no: Option<u32> = None;
    let mut is_part = false;
    let mut rc = 0;

    while let Some(ch) = chars.next() {
        let adj_up = adj_prev.next().unwrap();
        let adj_down = adj_next.next().unwrap();

        ch_prev = ch_curr;
        ch_curr = ch;
        ch_next = chars.peek().map(|c| *c).unwrap_or('.');;

        if let Some(v) = ch_curr.to_digit(10) {
            if !is_part {
                let adj_prev = !(ch_prev == '.' || ch_prev.is_digit(10));
                let adj_next = !(ch_next == '.' || ch_next.is_digit(10));

                is_part = adj_up || adj_down || adj_next || adj_prev;
            }

            if let Some(ref mut part_no) = curr_part_no {
                *part_no = *part_no * 10 + v;
            } else {
                curr_part_no = Some(v);
            }
        } else {
            if is_part {
                rc += curr_part_no.unwrap();
            }

            is_part = false;
            curr_part_no = None;
        }
    }

    if is_part {
        rc += curr_part_no.unwrap_or(0);
    }

    rc
}

fn sum_gears(prev: Option<&Vec<char>>, curr: &[char], next: Option<&Vec<char>>) -> u32 {
    let mut rc = 0;

    for (idx, ch) in curr.iter().enumerate() {
        if *ch == '*' {
            let (prev_count, prev_ratio) = if let Some(prev) = prev {
                read_adj_numbers(prev, idx)
            } else {
                (0, 0)
            };
            let (curr_count, curr_ratio) = read_adj_numbers(curr, idx);
            let (next_count, next_ratio) = if let Some(next) = next {
                read_adj_numbers(next, idx)
            } else {
                (0, 0)
            };

            if prev_count + curr_count + next_count == 2 {
                let mut ratio = 1;

                if prev_count > 0 {
                    ratio *= prev_ratio;
                }

                if curr_count > 0 {
                    ratio *= curr_ratio;
                }

                if next_count > 0 {
                    ratio *= next_ratio;
                }

                rc += ratio;
            }
        }
    }

    rc
}

pub fn solve1() -> u32 {
    let mut prev = None;
    let mut curr = None;
    let mut next = None;
    let mut sum = 0;

    for line in INPUT.lines() {
        prev = curr;
        curr = next;
        next = Some(line);

        if let Some(ref curr) = curr {
            sum = sum + sum_parts(prev, curr, next);
        }
    }

    prev = curr;
    curr = next;
    next = None;

    if let Some(ref curr) = curr {
        sum = sum + sum_parts(prev, curr, next);
    }

    sum
}

pub fn solve2() -> u32 {
    let mut prev = None;
    let mut curr = None;
    let mut next: Option<Vec<char>> = None;
    let mut sum = 0;

    for line in INPUT.lines() {
        prev = curr;
        curr = next;
        next = Some(line.chars().collect());

        if let Some(ref curr) = curr {
            sum = sum + sum_gears(prev.as_ref(), curr, next.as_ref());
        }
    }

    prev = curr;
    curr = next;
    next = None;

    if let Some(ref curr) = curr {
        sum = sum + sum_gears(prev.as_ref(), curr, next.as_ref());
    }

    sum
}
