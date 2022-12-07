use regex::Regex;

trait CrateMover<T> {
    fn move_crates(&self, amount: i32, src: &mut Vec<T>, dst: &mut Vec<T>);
}

struct CrateMover9000 {}

impl<T> CrateMover<T> for CrateMover9000 {
    fn move_crates(&self, amount: i32, src: &mut Vec<T>, dst: &mut Vec<T>) {
        for _ in 0 .. amount {
            let item = src.pop().expect("Stack is empty");
            dst.push(item);
        }
    }
}

struct CrateMover9001 {}

impl<T> CrateMover<T> for CrateMover9001 {
    fn move_crates(&self, amount: i32, src: &mut Vec<T>, dst: &mut Vec<T>) {
        let range_start = src.len() - amount as usize;
        let moved = src.drain(range_start .. );
        dst.extend(moved);
    }
}


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

fn run(crate_mover: impl CrateMover<char>) -> String {
    let (mut state, steps) = read_input();

    for (count, src, dst) in steps {
        let mut src_stack = std::mem::replace(&mut state[src], vec![]);
        let mut dst_stack = std::mem::replace(&mut state[dst], vec![]);
        crate_mover.move_crates(count, &mut src_stack, &mut dst_stack);
        state[src] = src_stack;
        state[dst] = dst_stack;
    }

    state.iter()
        .map(|x| x.last()
            .expect("No empty stacks are expected in the end"))
        .collect()
}

pub fn part_1() -> String {
    run(CrateMover9000 {})
}

pub fn part_2() -> String {
    run(CrateMover9001 {})
}
