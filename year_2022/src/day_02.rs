fn read_input() -> Vec<[u8; 2]> {
    let input = include_str!("input/day_02.txt");
    input.split("\n")
        .map(|x| x.as_bytes())
        .map(|x| [x[0] - "A".as_bytes()[0] + 1, x[2] - "X".as_bytes()[0] + 1])
        .collect()
}

pub fn part_1() -> i32 {
    let input = read_input();

    let mut score: i32 = 0;
    for row in input {
        let them = row[0] as i32;
        let me = row[1] as i32;

        score += me as i32;

        score += match them - me {
            1 | -2 => 0,
            0 => 3,
            _ => 6,
        }
    }

    score
}

pub fn part_2() -> i32 {
    let input = read_input();

    let mut score: i32 = 0;

    for row in input {
        let them = row[0] as i32;
        let me = row[1] as i32;

        score += (me - 1) as i32 * 3;

        score += match me {
            2 => them,
            1 => if them - 1 == 0 {3} else {them - 1},
            _ => if them + 1 == 4 {1} else {them + 1},
        };
    }

    score
}
