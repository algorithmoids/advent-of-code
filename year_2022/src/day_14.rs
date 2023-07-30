use std::collections::HashSet;

fn read_input() -> Vec<Vec<(i32, i32)>> {
    include_str!("input/day_14.txt")
        .split("\n")
        .map(|x| {
            x.split(" -> ")
                .map(|x| {
                    let p: Vec<_> = x.splitn(2, ',')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect();

                    (p[0], p[1])
                })
                .collect()
        })
        .collect()
}

fn get_blocks(input: Vec<Vec<(i32, i32)>>) -> (HashSet<(i32, i32)>, i32) {
    let mut blocks = HashSet::new();
    let mut lowest_point = 0;

    for row in input {
        for i in 0 .. row.len() - 1 {
            let (x0, y0) = row[i];
            let (x1, y1) = row[i+1];

            lowest_point = lowest_point.max(y0).max(y1);

            if x0 == x1 {
                for j in y0.min(y1) ..= y0.max(y1) {
                    blocks.insert((x0, j));
                }
            }
            else {
                for j in x0.min(x1) ..= x0.max(x1) {
                    blocks.insert((j, y0));
                }
            }
        }
    }

    (blocks, lowest_point)
}

pub fn part_1() -> usize {
    let (mut blocks, lowest_point) = get_blocks(read_input());
    let mut count = 0;

    loop {
        let mut sand = (500, 0);
        loop {
            if !blocks.contains(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            }
            else if !blocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            }
            else if !blocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            }
            else {
                blocks.insert(sand);
                break
            }

            if sand.1 > lowest_point {
                return count
            }
        }

        count += 1
    }
}

pub fn part_2() -> usize {
    let (mut blocks, lowest_point) = get_blocks(read_input());
    let mut count = 0;

    loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 == lowest_point + 1 {
                blocks.insert(sand);
                break
            }

            if !blocks.contains(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            }
            else if !blocks.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            }
            else if !blocks.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            }
            else {
                if sand == (500, 0) {
                    return count + 1
                }

                blocks.insert(sand);
                break
            }

        }

        count += 1
    }
}
