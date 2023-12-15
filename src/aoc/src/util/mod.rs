use std::iter::Peekable;
use std::str::Chars;

pub fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
        } else {
            break
        }
    }
}

pub fn skip_until<F>(chars: &mut Peekable<Chars>, f: F)
    where
        F: Fn(char) -> bool {
    while let Some(ch) = chars.peek() {
        if f(*ch) {
            break
        }

        chars.next();
    }
}

pub fn read_until<F>(chars: &mut Peekable<Chars>, f: F) -> String
    where
        F: Fn(char) -> bool {
    let mut s = String::new();

    while let Some(ch) = chars.peek() {
        if f(*ch) {
            break
        }

        s.push(*ch);
        chars.next();
    }

    s
}

pub fn read_u32(chars: &mut Peekable<Chars>) -> u32 {
    let mut num = 0;

    loop {
        if let Some(ch) = chars.next() {
            if let Some(v) = ch.to_digit(10) {
                num = num * 10 + v;
            } else {
                break
            }
        } else {
            break
        }
    }

    num
}

pub fn read_u64(chars: &mut Peekable<Chars>) -> u64 {
    let mut num = 0;

    loop {
        if let Some(ch) = chars.next() {
            if let Some(v) = ch.to_digit(10) {
                num = num * 10 + (v as u64);
            } else {
                break
            }
        } else {
            break
        }
    }

    num
}

pub fn read_i64(chars: &mut Peekable<Chars>) -> i64 {
    let mut num = 0;
    let mut negative = false;

    if let Some(ch) = chars.peek() {
        if *ch == '-' {
            negative = true;
            chars.next();
        }
    }

    loop {
        if let Some(ch) = chars.next() {
            if let Some(v) = ch.to_digit(10) {
                num = num * 10 + (v as i64);
            } else {
                break
            }
        } else {
            break
        }
    }

    if negative {
        -num
    } else {
        num
    }
}

pub fn ascii_read_as_u64_until<'a, I, F>(data: &mut Peekable<I>, mut f: F) -> u64
    where I: Iterator<Item = &'a u8>,
          F: FnMut(u8) -> bool {
    let mut rc = 0_u64;
    let mut offset = 0;

    while let Some(byte) = data.peek() {
        if f(**byte) {
            break;
        } else {
            rc += (**byte as u64) << offset;
            offset += 8;
        }

        data.next();
    }

    rc
}

pub fn ascii_read_usize<'a, I>(data: &mut Peekable<I>) -> usize
    where I: Iterator<Item = &'a u8> {
    let mut rc = 0_usize;

    while let Some(byte) = data.peek() {
        if **byte >= b'0' && **byte <= b'9' {
            rc = rc * 10 + byte.abs_diff(b'0') as usize;
        } else {
            break;
        }

        data.next();
    }

    rc
}
