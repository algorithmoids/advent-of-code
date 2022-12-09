use std::collections::HashSet;

fn read_input() -> Vec<char> {
    include_str!("input/day_06.txt").chars().collect()
}

pub fn part_1() -> usize {
    let input = read_input();

    for i in 4 .. input.len() {
        let last_chars: HashSet<_> = input[i - 4 .. i].iter().collect();

        if last_chars.len() == 4 {
            return i
        }
    }

    panic!("Didn't find an answer")
}