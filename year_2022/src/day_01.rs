use std::collections::LinkedList;
use std::fs;
use std::fs::File;

fn read_input() -> Vec<&'static str> {
    let input = include_str!("input/day_01.txt");
    input.split("\n").collect()
}

fn count_top(n: usize) -> i32 {
    let input = read_input();

    let mut top_caloeris = Vec::new();

    let mut current_calories = 0;

    for row in input {
        if row == "" {
            top_caloeris.push(current_calories);
            current_calories = 0;
        }
        else {
            current_calories += row.parse::<i32>().expect("Failed parsing input number");
        }
    }

    top_caloeris.push(current_calories);

    top_caloeris.sort();
    top_caloeris.iter().rev().take(n).sum()
}

pub(crate) fn part_1() -> i32 {
    count_top(1)
}

pub(crate) fn part_2() -> i32 {
    count_top(3)
}


