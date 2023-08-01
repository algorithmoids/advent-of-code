use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use regex::Regex;

fn read_input() -> Vec<Vec<i32>> {
    let parser = Regex::new(r"Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)")
        .unwrap();

    let mut sensors = vec![];

    let input = include_str!("input/day_15.txt");

    for row in input.split('\n') {
        let coordinates: Vec<_> = parser.captures(row).unwrap()
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
            .collect();

        sensors.push(coordinates)
    }

    sensors
}

pub fn part_1() -> i32 {
    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();

    let y_row = 2000000;

    for row in read_input() {
        sensors.insert((row[0], row[1]));
        beacons.insert((row[2], row[3]));
    }

    let mut ranges = vec![];

    for row in read_input() {
        let [sx, sy, bx, by] = row.deref()
            else { panic!("Not enough values") };

        let distance = (sx - bx).abs() + (sy - by).abs();

        if sy < &y_row && sy + distance >= y_row {
            let len_x = sy + distance - y_row;
            ranges.push((sx - len_x, sx + len_x));
        }

        if sy > &y_row && sy - distance <= y_row {
            let len_x = y_row - (sy - distance);
            ranges.push((sx - len_x, sx + len_x));
        }
    }

    let merged = merge(ranges);
    let mut count = merged.iter().map(|(a, b)| b - a + 1).sum();

    for (sx, sy) in sensors.iter().chain(beacons.iter()) {
        if sy == &y_row && merged.binary_search_by(|(a, b)|
                if b < sx { Less } else if a > sx { Greater } else { Equal }).is_ok() {
            count -= 1;
        }
    }

    count
}

fn merge(ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut uncrossing_ranges: Vec<(i32, i32)> = vec![];

    for (mut a, mut b) in ranges {
        let index_start = uncrossing_ranges.binary_search_by(|x|
                if x.1 > a { Greater } else if x.1 < a { Less } else {Equal} );

        let index_end = uncrossing_ranges.binary_search_by(|x|
                if x.0 > b { Greater } else if x.0 < b { Less } else {Equal} );

        if index_start == index_end && index_start.is_err() {
            uncrossing_ranges.insert(index_start.unwrap_err(), (a, b))
        }
        else {
            let index = index_start.unwrap_or_else(|x| x);

            a = a.min(uncrossing_ranges[index].0);

            loop {
                if uncrossing_ranges.len() == index || uncrossing_ranges[index].0 > b {
                    uncrossing_ranges.insert(index, (a, b));
                    break
                }
                else {
                    b = b.max(uncrossing_ranges[index].1);
                    uncrossing_ranges.remove(index);
                }
            }
        }

    }
    uncrossing_ranges
}

#[derive(Debug)]
struct Segment {
    root: i32,
    top: i32,
    bottom: i32,
}

#[derive(Debug)]
struct Bounds {
    left: HashMap<i32, Vec<Segment>>,
    right: HashMap<i32, Vec<Segment>>,
    top: HashMap<i32, Vec<Segment>>,
    bottom: HashMap<i32, Vec<Segment>>,
}

pub fn part_2() -> i64 {
    let bounds = get_bounds();

    let ascending = get_narrow_ranges(bounds.left, bounds.right);
    let descending = get_narrow_ranges(bounds.top, bounds.bottom);

    for a in ascending.iter() {
        for d in descending.iter() {
            if a.top.min(a.top) > a.bottom.max(d.bottom) {
                let x = ((a.root + d.root) / 2) as i64;
                let y = ((a.root - d.root) / 2) as i64;
                return x * 4000000 + y;
            }
        }
    }

    unreachable!()
}

fn get_bounds() -> Bounds {

    let mut bounds = Bounds {
        left: HashMap::new(),
        right: HashMap::new(),
        top: HashMap::new(),
        bottom: HashMap::new(),
    };

    for row in read_input() {
        let [sx, sy, bx, by] = row[..4]
            else { panic!("Input parser failed") };

        let distance = (sx - bx).abs() + (sy - by).abs();

        add_bound(&mut bounds.left, Segment {
            root: sx - distance + sy,
            top: sy,
            bottom: sy - distance,
        });

        add_bound(&mut bounds.right, Segment {
            root: sx + distance + sy,
            top: sy + distance,
            bottom: sy,
        });

        add_bound(&mut bounds.top, Segment {
            root: sx - distance - sy,
            top: sy + distance,
            bottom: sy,
        });

        add_bound(&mut bounds.bottom, Segment {
            root: sx + distance - sy,
            top: sy,
            bottom: sy - distance,
        });
    }

    bounds

}

fn add_bound(side: &mut HashMap<i32, Vec<Segment>>, bound: Segment) {
    if side.contains_key(&bound.root) {
        side.get_mut(&bound.root).unwrap().push(bound);
    }
    else {
        side.insert(bound.root, vec![bound]);
    }
}

fn get_narrow_ranges(left: HashMap<i32, Vec<Segment>>, right: HashMap<i32, Vec<Segment>>) -> Vec<Segment> {
    let empty_vec = vec![];
    let mut segments = vec![];

    for (left_root, left) in left {
        let right_segments = right.get(&(left_root - 2))
            .unwrap_or(&empty_vec);

        for left_segment in left {
            for right_segment in right_segments {
                if left_segment.top.min(right_segment.top) >= left_segment.bottom.max(right_segment.bottom) {
                    segments.push(Segment {
                        root: left_root - 1,
                        top: left_segment.top.min(right_segment.top),
                        bottom: left_segment.bottom.max(right_segment.bottom)
                    });
                }
            }
        }
    }

    segments
}
