use crate::util::{ascii_read_as_u64_until, ascii_read_usize};

static INPUT: &[u8] = include_bytes!("input");
//static INPUT: &[u8] = include_bytes!("input_sample");

#[derive(Copy, Clone)]
struct Lens {
    label: u64,
    focal_length: usize
}

fn hash(byte: &u8, curr: u32) -> u32 {
    let byte = *byte as u32;

    ((curr + byte) * 17) % 256
}

pub fn solve1() -> u32 {
    let mut rc = 0;
    let mut curr = 0;

    for byte in INPUT {
        if *byte != b'\n' {
            if *byte == b',' {
                rc += curr;
                curr = 0;
            } else {
                curr = hash(byte, curr);
            }
        }
    }

    rc + curr
}

pub fn solve2() -> usize {
    let mut map: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    let mut iter = INPUT.iter().peekable();

    while iter.peek().is_some() {
        let mut label_hash = 0;
        let label = ascii_read_as_u64_until(&mut iter, |byte| {
            if byte == b'-' || byte == b'=' {
                true
            } else {
                label_hash = hash(&byte, label_hash);
                false
            }
        });

        if let Some(b'-') = iter.next() {
            map[label_hash as usize].retain(|l| l.label != label);
        } else {
            let mut found = false;
            let focal_length = ascii_read_usize(&mut iter);

            for l in map[label_hash as usize].iter_mut() {
                if l.label == label {
                    l.focal_length = focal_length;
                    found = true;
                    break;
                }
            }

            if !found {
                map[label_hash as usize].push(Lens { label, focal_length });
            }
        }

        iter.next();
    }

    let mut rc = 0;

    for (idx, v) in map.iter().enumerate() {
        for (slot, l) in v.iter().enumerate() {
            rc += (idx + 1) * (slot + 1) * l.focal_length;
        }
    }

    rc
}
