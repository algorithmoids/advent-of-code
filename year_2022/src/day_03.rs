use std::collections::HashSet;

fn read_input() -> Vec<&'static str> {
    let input = include_str!("input/day_03.txt");
    input.split("\n").collect()
}

pub fn part_1() -> i32 {
    let mut priority: i32 = 0;

    for items in read_input() {
        let pocket_size = items.len() / 2;
        let pocket_1 = HashSet::<char>::from_iter(items.chars().take(pocket_size));
        let pocket_2 = HashSet::<char>::from_iter(items.chars().rev().take(pocket_size));
        let common = pocket_1.intersection(&pocket_2).last().expect("No common itmes");

        priority += if common > &'Z' {
            (*common as i32) - 'a' as i32 + 1
        }
        else {
            *common as i32 - 'A' as i32 + 27
        }
    }

    priority
}
