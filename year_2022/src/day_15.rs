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

pub fn part_1() -> usize {
    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();

    let y_row = 2000000;

    let mut on_line = HashSet::new();

    for row in read_input() {
        sensors.insert((row[0], row[1]));
        beacons.insert((row[2], row[3]));
    }

    for row in read_input() {
        let [sx, sy, bx, by] = row.deref()
            else { panic!("Not enough values") };

        let distance = (sx - bx).abs() + (sy - by).abs();

        if sy < &y_row && sy + distance >= y_row {
            let len_x = sy + distance - y_row;
            for x in sx - len_x ..= sx + len_x {
                if !sensors.contains(&(x, y_row)) && !beacons.contains(&(x, y_row)) {
                    on_line.insert(x);
                }
            }
        }

        if sy > &y_row && sy - distance <= y_row {
            let len_x = y_row - (sy - distance);
            for x in sx - len_x ..= sx + len_x {
                if !sensors.contains(&(x, y_row)) && !beacons.contains(&(x, y_row)) {
                    on_line.insert(x);
                }
            }
        }
    }

    on_line.len()
}

pub fn _part_1_slow() -> usize {
    let mut empty = HashSet::new();
    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();

    let y = 2000000;

    let mut on_line = HashSet::new();

    for row in read_input() {
        sensors.insert((row[0], row[1]));
        beacons.insert((row[2], row[3]));
    }

    for row in read_input() {
        let [sx, sy, bx, by] = row.deref()
            else { panic!("Not enough values") };

        let distance = (sx - bx).abs() + (sy - by).abs();

        for i in 1 ..= distance {
            for j in 0 .. i {
                let steps = [
                    (sx - i + j, sy + j),
                    (sx + j, sy + i - j),
                    (sx + i - j, sy - j),
                    (sx - j, sy - i + j),
                ];

                for p in steps {

                    if !sensors.contains(&p) && !beacons.contains(&p) {
                        empty.insert(p);

                        if p.1 == y {
                            on_line.insert(p.0);
                        }
                    }
                }
            }
        }
    }

    on_line.len()
}
