fn read_input() -> Vec<&'static str> {
    let input = include_str!("input/day_01.txt");
    input.split("\n").collect()
}

pub(crate) fn part_1() -> u32 {
    let mut sum = 0;

    for row in read_input() {

        let mut digits = row.chars().into_iter()
            .filter(|x| x.is_digit(10));

        let first = digits.next().unwrap().to_digit(10).unwrap();
        let last = digits.last().map(|x| x.to_digit(10).unwrap()).unwrap_or(first);

        sum += first * 10 + last

    }

    sum
}
