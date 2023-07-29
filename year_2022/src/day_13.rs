use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::vec;

#[derive(Debug)]
enum Signal {
    List(Vec<Signal>),
    Value(u32)
}


fn read_input() -> Vec<(Signal, Signal)> {
    include_str!("input/day_13.txt")
        .split("\n\n")
        .map(|x| {
            let Some((a, b)) = x.split_once('\n') else { panic!("Should be two lines") };
            let a: Vec<char> = a.chars().into_iter().collect();
            let b: Vec<char> = b.chars().into_iter().collect();

            let (_, a) = parse_subsignal(&a[1..]);
            let (_, b) = parse_subsignal(&b[1..]);

            (a, b)
        })
        .collect()
}

fn parse_subsignal(chars: &[char]) -> (usize, Signal) {
    let mut signal: Vec<Signal> = vec![];
    let mut value = None;
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            ',' => {
                value.map(|x| signal.push(Signal::Value(x)));
                value = None;
            },
            '0' ..= '9' => {
                value = value.map_or(c.to_digit(10),
                                     |x| Some(x * 10 + c.to_digit(10).unwrap()));
            },
            ']' => {
                value.map(|x| signal.push(Signal::Value(x)));
                return (i, Signal::List(signal))
            },
            '[' => {
                let (jump, subsignal) = parse_subsignal(&chars[i+1 .. ]);
                signal.push(subsignal);
                i += jump + 1;
            },
            _ => panic!()
        }

        i += 1;
    }

    unreachable!("It should always be a `]`")
}

fn compare(a: &Signal, b: &Signal) -> Ordering {
    if let (Signal::Value(a), Signal::Value(b)) = (a, b) {
        if *a < *b {
            return Less
        }
        else if *a > *b {
            return Greater
        }
        return Equal
    }

    let glob_a;

    let a = match a {
        Signal::List(al) => al,
        Signal::Value(av) => {
            glob_a = vec![Signal::Value(*av)];
            &glob_a
        }
    };

    let glob_b;

    let b = match b {
        Signal::List(bl) => bl,
        Signal::Value(bv) => {
            glob_b = vec![Signal::Value(*bv)];
            &glob_b
        }
    };

    for i in 0 .. a.len().max(b.len()) {
        if a.len() == i {
            return Less
        }
        if b.len() == i {
            return Greater
        }

        let subcompare = compare(&a[i], &b[i]);
        if subcompare != Equal {
            return subcompare
        }
    }

    Equal
}

pub fn part_1() -> usize {
    let mut checksum = 0;
    for (i, (a, b)) in read_input().into_iter().enumerate() {
        if compare(&a, &b) == Less {
            checksum += i + 1;
        }
    }

    return checksum
}

pub fn part_2() -> usize {
    let mut signals: Vec<_> = read_input().into_iter()
        .map(|(a, b)| vec![a, b])
        .flatten()
        .collect();

    signals.sort_by(compare);

    let s2: Vec<char> = "[[2]]".chars().into_iter().collect();
    let (_, s2) = parse_subsignal(&s2[1..]);

    let s6: Vec<char> = "[[6]]".chars().into_iter().collect();
    let (_, s6) = parse_subsignal(&s6[1..]);

    let s2p = signals.binary_search_by(|x| compare(x, &s2));
    let s6p = signals.binary_search_by(|x| compare(x, &s6));

    return (s2p.unwrap_err() + 1) * (s6p.unwrap_err() + 2)
}
