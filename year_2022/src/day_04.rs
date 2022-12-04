use regex::Regex;

fn read_input() -> Vec<Vec<i32>> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    let input = include_str!("input/day_04.txt");
    input.split("\n")
        .map(|x| re.captures(x).expect("Failed to parse input"))
        .map(|cpatures| cpatures.iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<i32>().expect("Failed parsing input number"))
            .collect::<Vec<_>>())
        .collect()
}

pub fn part_1() -> i32 {
    let mut including = 0;

    for input in read_input() {
        if (input[0] >= input[2] && input[1] <= input[3]) || (input[2] >= input[0] && input[3] <= input[1]) {
            including += 1;
        }
    }

    including
}

pub fn part_2() -> i32 {
    let mut overlapping = 0;

    for input in read_input() {
        if (input[2] <= input[0] && input[0] <= input[3]) || (input[0] <= input[2] && input[2] <= input[1]) {
            overlapping += 1;
        }
    }

    overlapping
}
