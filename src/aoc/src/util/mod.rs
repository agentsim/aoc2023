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

pub fn read_number(chars: &mut Peekable<Chars>) -> u32 {
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
