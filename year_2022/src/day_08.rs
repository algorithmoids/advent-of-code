use std::collections::HashSet;

fn read_input() -> Vec<Vec<u8>> {
    include_str!("input/day_08.txt")
        .split("\n")
        .map(|x| x.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
        ).collect()
}

pub fn part_1() -> usize {
    let input = read_input();

    let top_to_bottom = input.iter().map(|x| x.iter());
    let mut visible: HashSet<_> = get_visible_from_edge(top_to_bottom)
        .into_iter().collect();

    let bottom_to_top = input.iter().map(|x| x.iter()).rev();
    let visible_bottom = get_visible_from_edge(bottom_to_top)
        .into_iter()
        .map(|(x, y)| (input[0].len() - 1 - x, y));

    visible.extend(visible_bottom);

    let mut transposed = vec![];
    for i in 0 .. input[0].len() {
        let mut row = vec![];
        for j in 0..input.len() {
            row.push(input[j][i])
        }
        transposed.push(row);
    }

    let left_to_right = transposed.iter().map(|x| x.iter());
    let visible_left = get_visible_from_edge(left_to_right)
        .into_iter()
        .map(|(x, y)| (y, x));

    visible.extend(visible_left);

    let right_to_left = transposed.iter().map(|x| x.iter()).rev();
    let visible_to_right = get_visible_from_edge(right_to_left)
        .into_iter()
        .map(|(x, y)| (y, input.len() - 1 - x));

    visible.extend(visible_to_right);

    visible.len()
}

fn get_visible_from_edge<'a>(mut forest: impl Iterator<Item = impl Iterator<Item = &'a u8>>) -> Vec<(usize, usize)> {

    let mut wave: Vec<_> = forest.next().unwrap().collect();
    let mut visible: Vec<(usize, usize)> = wave.iter().enumerate().map(|(i, _)| (0, i)).collect();

    for (i, row) in forest.enumerate() {
        for (j, height) in row.enumerate() {
            if height > wave[j] {
                visible.push((i+1, j));
                wave[j] = height;
            }
        }
    }

    return visible
}
