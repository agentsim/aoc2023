use anyhow::{bail, Result};
use std::str::Chars;
use std::iter::Peekable;
use super::util::{read_u32, skip_whitespace};

static INPUT: &str = include_str!("input");

enum Colour {
    Red,
    Green,
    Blue
}

fn skip(chars: &mut Peekable<Chars>, mut n: usize) {
    while n > 0 && chars.next().is_some() {
        n = n - 1
    }
}

fn read_colour(chars: &mut Peekable<Chars>) -> Result<Colour> {
    if let Some(ch) = chars.next() {
        if ch == 'r' {
            skip(chars, 2);
            Ok(Colour::Red)
        } else if ch == 'g' {
            skip(chars, 4);
            Ok(Colour::Green)
        } else if ch == 'b' {
            skip(chars, 3);
            Ok(Colour::Blue)
        } else {
            bail!("Unexpected char while reading colour: {}", ch);
        }
    } else {
        bail!("Unexpected end of input while reading colour");
    }
}

fn parse_result(chars: &mut Peekable<Chars>) -> Result<(u32, Colour)> {
    let number = read_u32(chars);

    skip_whitespace(chars);

    let colour = read_colour(chars)?;

    Ok((number, colour))
}

fn parse_draw(chars: &mut Peekable<Chars>) -> Result<(u32, u32, u32)> {
    let mut rc = (0, 0, 0);
    let (number, colour) = parse_result(chars)?;

    match colour {
        Colour::Red => rc.0 = number,
        Colour::Green => rc.1 = number,
        Colour::Blue => rc.2 = number,
    }

    skip_whitespace(chars);

    if let Some(ch) = chars.next() {
        if ch == ';' {
            return Ok(rc)
        }
    } else {
        return Ok(rc)
    }

    skip_whitespace(chars);

    let (number, colour) = parse_result(chars)?;

    match colour {
        Colour::Red => rc.0 = number,
        Colour::Green => rc.1 = number,
        Colour::Blue => rc.2 = number,
    }

    skip_whitespace(chars);

    if let Some(ch) = chars.next() {
        if ch == ';' {
            return Ok(rc)
        }
    } else {
        return Ok(rc)
    }

    skip_whitespace(chars);

    let (number, colour) = parse_result(chars)?;

    match colour {
        Colour::Red => rc.0 = number,
        Colour::Green => rc.1 = number,
        Colour::Blue => rc.2 = number,
    }

    skip_whitespace(chars);

    if let Some(ch) = chars.peek() {
        if *ch == ';' {
            chars.next();
        }
    }

    Ok(rc)
}

fn parse_game(input: &str) -> Result<(bool, u32)> {
    let mut chars = input.chars().peekable();

    skip(&mut chars, 5);
    let game_id = read_u32(&mut chars);

    skip_whitespace(&mut chars);

    while chars.peek().is_some() {
        let (r, g, b) = parse_draw(&mut chars)?;

        if r > 12 || g > 13 || b > 14 {
            return Ok((false, game_id))
        }

        skip_whitespace(&mut chars);
    }

    Ok((true, game_id))
}

fn parse_game_2(input: &str) -> Result<(u32)> {
    let mut chars = input.chars().peekable();

    skip(&mut chars, 5);
    let game_id = read_u32(&mut chars);
    let mut max = (0, 0, 0);

    skip_whitespace(&mut chars);

    while chars.peek().is_some() {
        let (r, g, b) = parse_draw(&mut chars)?;

        if r > max.0 {
            max.0 = r
        }

        if g > max.1 {
            max.1 = g
        }

        if b > max.2 {
            max.2 = b
        }

        skip_whitespace(&mut chars);
    }

    Ok(max.0 * max.1 * max.2)
}

pub fn solve1() -> Result<u32> {
    let mut rc = 0;

    for line in INPUT.lines() {
        let (possible, id) = parse_game(line)?;

        if possible {
            rc = rc + id;
        }
    }

    Ok(rc)
}

pub fn solve2() -> Result<u32> {
    let mut rc = 0;

    for line in INPUT.lines() {
        rc = rc + parse_game_2(line)?;
    }

    Ok(rc)
}
