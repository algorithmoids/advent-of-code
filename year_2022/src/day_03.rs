use std::collections::HashSet;

fn read_input() -> Vec<&'static str> {
    let input = include_str!("input/day_03.txt");
    input.split("\n").collect()
}

pub fn part_1() -> i32 {
    let mut priority: i32 = 0;

    for items in read_input() {
        let pocket_size = items.len() / 2;
        let pockets = vec![
            items.chars().take(pocket_size).collect(),
            items.chars().rev().take(pocket_size).collect(),
        ];

        let common = get_common_item(pockets);

        priority += get_item_priority(&common)
    }

    priority
}

pub fn part_2() -> i32 {
    let mut priority: i32 = 0;

    let input = read_input();

    for group in (0 .. input.len()).step_by(3) {
        let pockets = vec![
                input[group], input[group + 1], input[group + 2]
            ]
            .into_iter()
            .map(|x| x.chars().collect())
            .collect();

        let common = get_common_item(pockets);

        priority += get_item_priority(&common)
    }

    priority
}

fn get_common_item(groups: Vec<Vec<char>>) -> char {
    groups.into_iter()
        .map(|x| HashSet::<char>::from_iter(x))
        .reduce(|a, b|
            a.intersection(&b).into_iter().map(|x| *x).collect()
        )
        .expect("No common items")
        .into_iter().collect::<Vec<_>>()[0]
}

fn get_item_priority(item: &char) -> i32 {
    if item > &'Z' {
        (*item as i32) - 'a' as i32 + 1
    }
    else {
        *item as i32 - 'A' as i32 + 27
    }
}
