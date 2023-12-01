static INPUT: &str = include_str!("input");
struct Recognizer {
    seqs: Vec<Vec<char>>,
    current_seqs: Vec<(usize, usize)>
}

impl Recognizer {
    pub fn new() -> Recognizer {
        Recognizer {
            seqs: vec![
                vec!['o', 'n', 'e'],
                vec!['t', 'w', 'o'],
                vec!['t', 'h', 'r', 'e', 'e'],
                vec!['f', 'o', 'u', 'r'],
                vec!['f', 'i', 'v', 'e'],
                vec!['s', 'i', 'x'],
                vec!['s', 'e', 'v', 'e', 'n'],
                vec!['e', 'i', 'g', 'h', 't'],
                vec!['n', 'i', 'n', 'e']
            ],
            current_seqs: Vec::with_capacity(100)
        }
    }

    pub fn read(&mut self, c: char) -> Option<usize> {
        if let Some(v) = c.to_digit(10) {
            self.current_seqs.clear();
            Some(v as usize)
        } else {
            let mut rc = None;

            self.current_seqs.retain_mut(|(seq_idx, ref mut pos)| {
                if self.seqs[*seq_idx][*pos] == c {
                    if *pos == self.seqs[*seq_idx].len() - 1 {
                        rc = Some(*seq_idx + 1);
                        false
                    } else {
                        *pos = *pos + 1;
                        true
                    }
                } else {
                    false
                }
            });

            for (idx, seq) in self.seqs.iter().enumerate() {
                if seq[0] == c {
                    self.current_seqs.push((idx, 1))
                }
            }

            rc
        }
    }
}

pub fn solve1() {
    let mut sum = 0;

    for line in INPUT.lines() {
        let mut line_val = 0;

        for c in line.chars() {
            if let Some(v) = c.to_digit(10) {
                line_val = v * 10;
                break;
            }
        }

        for c in line.chars().rev() {
            if let Some(v) = c.to_digit(10) {
                line_val = line_val + v;
                break;
            }
        }

        sum = sum + line_val
    }

    println!("AOC 1.1 - Solution is: {}", sum)
}

pub fn solve2() {
    let mut sum = 0;

    for line in INPUT.lines() {
        let mut digits = Vec::with_capacity(100);
        let mut recognizer = Recognizer::new();

        for c in line.chars() {
            if let Some(v) = recognizer.read(c) {
                digits.push(v);
            }
        }

        if !digits.is_empty() {
            let l = digits.first().unwrap() * 10 + digits.last().unwrap();

            sum = sum + l
        }
    }

    println!("AOC 1.2 - Solution is: {}", sum)
}
