use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use crate::util::{read_u64, skip_whitespace};

static INPUT: &str = include_str!("input");

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    vals: [u32; 5],
    bid: u64,
    hand_type: HandType
}

impl Hand {
    fn new(cards: [char; 5], bid: u64, joker: bool) -> Hand {
        let vals = [
            value(&cards[0], joker),
            value(&cards[1], joker),
            value(&cards[2], joker),
            value(&cards[3], joker),
            value(&cards[4], joker),
        ];
        Hand {
            vals, bid, hand_type: Self::hand_type(&cards, joker)
        }
    }

    fn hand_type(cards: &[char; 5], joker: bool) -> HandType {
        let mut combos = HashMap::with_capacity(5);

        for c in cards {
            if let Some(v) = combos.get_mut(&c) {
                *v = *v + 1;
            } else {
                combos.insert(c, 1);
            }
        }

        if joker {
            let j_count = combos.get(&'J').map(|v| *v).unwrap_or(0);

            if j_count > 0 && j_count < 5 {
                let mut best_card = None;
                let mut best_count = 0;

                combos.remove(&'J');

                for ((k, v)) in combos.iter() {
                    if *v > best_count {
                        best_count = *v;
                        best_card = Some(*k);
                    }
                }

                if let Some(best_card) = best_card {
                    if let Some(v) = combos.get_mut(&best_card) {
                        *v = *v + j_count;
                    }
                }
            }
        }

        let len = combos.len();

        if len == 5 {
            return HandType::HighCard
        } else if len == 4 {
            HandType::OnePair
        } else if len == 1 {
            HandType::FiveOfAKind
        } else {
            let mut have_3 = false;
            let mut have_2 = false;
            let mut hand_type = HandType::TwoPair;

            for (_, count) in combos.iter() {
                if *count == 4 {
                    return HandType::FourOfAKind;
                } else if *count == 3 {
                    have_3 = true;
                } else if *count == 2 {
                    have_2 = true;
                }
            }

            if have_3 && have_2 {
                hand_type = HandType::FullHouse;
            } else if have_3 && !have_2 {
                hand_type = HandType::ThreeOfAKind;
            }

            hand_type
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.vals == other.vals
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let h1 = &self.hand_type;
        let h2 = &other.hand_type;

        let h_ord = h1.cmp(&h2);

        if h_ord == Ordering::Equal {
            for i in 0..5 {
                let v_ord = self.vals[i].cmp(&other.vals[i]);

                if v_ord != Ordering::Equal {
                    return Some(v_ord);
                }
            }

            Some(Ordering::Equal)
        } else {
            Some(h_ord)
        }
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn value(c: &char, joker: bool) -> u32 {
    if let Some(v) = c.to_digit(10) {
        v
    } else if *c == 'T' {
        10
    } else if *c == 'J' {
        if joker {
            1
        } else {
            11
        }
    } else if *c == 'Q' {
        12
    } else if *c == 'K' {
        13
    } else {
        14
    }
}

fn solve(joker: bool) -> usize {
    let mut lines = INPUT.lines();
    let mut hands = BTreeSet::new();
    let mut rc = 0;

    for line in lines {
        let mut chars = line.chars().peekable();
        let cards = [
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        ];

        skip_whitespace(&mut chars);
        let bid = read_u64(&mut chars);

        hands.insert(Hand::new(cards, bid, joker));
    }

    for (idx, hand) in hands.iter().enumerate() {
        rc = rc + (idx + 1) * (hand.bid as usize);
    }

    rc

}

pub fn solve1() -> usize {
    solve(false)
}

pub fn solve2() -> usize {
    solve(true)
}
