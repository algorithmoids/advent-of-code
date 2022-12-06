use regex::Regex;

fn read_input() -> (Vec<Vec<char>>, Vec<(i32, usize, usize)>) {
    let input = include_str!("input/day_05.txt");

    let parts = input.split("\n\n").collect::<Vec<_>>();
    let state: Vec<_> = parts[0].split("\n")
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();

    let n_stacks = (state.last().expect("No state rows").len() + 2) / 4;
    let mut stacks = vec![];

    for i in 0 .. n_stacks {
        let mut stack = vec![];
        for row in state.iter().rev().skip(1) {
            if row.len() >= i * 4 && row[i * 4 + 1] != ' ' {
                stack.push(row[i * 4 + 1])
            }
        }

        stacks.push(stack)
    }


    let actions = parts[1].split("\n");

    let mut steps = vec![];
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    for action in actions {
        let captures = re.captures(action).expect("Failed parsing action");
        let capture = &captures;

        steps.push((
            capture[1].parse::<i32>().expect("Failed to parse count"),
            captures[2].parse::<usize>().expect("Failed to parse source") - 1,
            captures[3].parse::<usize>().expect("Failed to parse target") - 1
        ))
    }
    (stacks, steps)

}

pub fn part_1() -> String {
    let (mut state, steps) = read_input();

    for (count, src, dst) in steps {
        for _ in 0 .. count {
            let moved = state[src].pop().expect("Stack is empty");
            state[dst].push(moved);
        }
    }

    state.iter()
        .map(|x| x.last()
            .expect("No empty stacks are expected in the end"))
        .collect()
}
