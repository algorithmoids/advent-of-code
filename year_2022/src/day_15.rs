use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;
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
