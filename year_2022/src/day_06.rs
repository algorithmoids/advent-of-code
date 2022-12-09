use std::collections::HashSet;

fn read_input() -> Vec<char> {
    include_str!("input/day_06.txt").chars().collect()
}

fn get_start(unique: usize) -> usize {
    let input = read_input();

    for i in unique .. input.len() {
        let last_chars: HashSet<_> = input[i - unique .. i].iter().collect();

        if last_chars.len() == unique {
            return i
        }
    }

    panic!("Didn't find an answer")
}

pub fn part_1() -> usize {
    get_start(4)
}

pub fn part_2() -> usize {
    get_start(14)
}
