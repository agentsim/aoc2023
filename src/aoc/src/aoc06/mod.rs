use crate::util::{read_u32, skip_until, skip_whitespace};

static INPUT: &str = include_str!("input");

fn read_numbers(s: &str) -> Vec<u32> {
    let mut chars = s.chars().peekable();
    let mut rc = Vec::with_capacity(100);

    skip_until(&mut chars, |ch| ch == ':');
    chars.next();

    while chars.peek().is_some() {
        skip_whitespace(&mut chars);
        rc.push(read_u32(&mut chars));
        skip_whitespace(&mut chars);
    }

    rc
}

fn read_numbers_2(s: &str) -> f64 {
    let mut rc: f64 = 0.;

    for ch in s.chars() {
        if let Some(v) = ch.to_digit(10) {
            rc = rc * 10. + (v as f64);
        }
    }

    rc
}

fn ways_to_win(time: f64, record: f64) -> u32 {
    // distance = (time - accel) * accel
    // distance = -accel^2 + time*accel
    // surpass record = -accel^2 + time*accel - record
    // 0 = ax^2 + bx + c
    let a: f64 = - 1.0;
    let b: f64 = time;
    let c: f64 = -(record + 1.);

    // intercepts = (-b +/- sqrt(b^2 - 4ac)) / 2a
    let i1 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let i2 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let low = i1.min(i2).ceil();
    let high = i1.max(i2).floor();

    (high - low + 1.) as u32
}

pub fn solve1() -> u32 {
    let mut lines = INPUT.lines();
    let times = read_numbers(lines.next().unwrap());
    let distances = read_numbers(lines.next().unwrap());
    let mut rc: u32 = 1;

    for (t, r) in times.iter().zip(distances.iter()) {
        let time = f64::from(*t);
        let record = f64::from(*r);

        rc *= ways_to_win(time, record);
    }

    rc
}

pub fn solve2() -> u32 {
    let mut lines = INPUT.lines();
    let time = read_numbers_2(lines.next().unwrap());
    let distance = read_numbers_2(lines.next().unwrap());

    ways_to_win(time, distance)
}
