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

fn compare(a: &Signal, b: &Signal) -> i32 {
    if let (Signal::Value(a), Signal::Value(b)) = (a, b) {
        if *a < *b {
            return -1
        }
        else if *a > *b {
            return 1
        }
        return 0
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
            return -1
        }
        if b.len() == i {
            return 1
        }

        let subcompare = compare(&a[i], &b[i]);
        if subcompare != 0 {
            return subcompare
        }
    }

    0
}

pub fn part_1() -> usize {
    let mut checksum = 0;
    for (i, (a, b)) in read_input().into_iter().enumerate() {
        if compare(&a, &b) == -1 {
            checksum += i + 1;
        }
    }

    return checksum
}
