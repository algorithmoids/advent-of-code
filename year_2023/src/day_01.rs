use std::collections::HashMap;
use regex::Regex;


fn read_input() -> Vec<&'static str> {
    let input = include_str!("input/day_01.txt");
    input.split("\n").collect()
}

pub(crate) fn part_1() -> u32 {
    let mapping = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    get_sum(mapping)
}

pub(crate) fn part_2() -> u32 {
    let mapping = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    get_sum(mapping)
}

fn get_sum(mapping: HashMap<&str, u8>) -> u32 {
    let mut sum: u32 = 0;

    let numbers_re = mapping.iter().map(|(k, _)| *k).collect::<Vec<_>>().join("|");
    let first_re = Regex::new(&format!(".*?({}).*", numbers_re)).unwrap();
    let last_re = Regex::new(&format!(".*({}).*", numbers_re)).unwrap();

    for row in read_input() {
        let first = get_part(row, &first_re, &mapping);
        let last = get_part(row, &last_re, &mapping);

        sum += (first * 10 + last) as u32;
    }
    sum
}

fn get_part(text: &str, pattern: &Regex, map: &HashMap<&str, u8>) -> u8 {
    let captures = pattern.captures(text).unwrap();
    let number = captures.get(1).unwrap().as_str();
    return map[number]
}
