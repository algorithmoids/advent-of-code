use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_08;
mod day_13;
mod day_14;
mod day_15;

fn main() {
    run_measure("Day 1.1", &day_01::part_1);
    run_measure("Day 1.2", &day_01::part_2);
    run_measure("Day 2.1", &day_02::part_1);
    run_measure("Day 2.2", &day_02::part_2);
    run_measure("Day 3.1", &day_03::part_1);
    run_measure("Day 3.2", &day_03::part_2);
    run_measure("Day 4.1", &day_04::part_1);
    run_measure("Day 4.2", &day_04::part_2);
    run_measure("Day 5.1", &day_05::part_1);
    run_measure("Day 5.2", &day_05::part_2);
    run_measure("Day 6.1", &day_06::part_1);
    run_measure("Day 6.2", &day_06::part_2);
    run_measure("Day 8.1", &day_08::part_1);
    run_measure("Day 8.2", &day_08::part_2);
    run_measure("Day 13.1", &day_13::part_1);
    run_measure("Day 13.2", &day_13::part_2);
    run_measure("Day 14.1", &day_14::part_1);
    run_measure("Day 14.2", &day_14::part_2);
    run_measure("Day 15.1", &day_15::part_1);
    run_measure("Day 15.2", &day_15::part_2);
}

fn run_measure<T: std::fmt::Display>(name: &str, function: &dyn Fn() -> T) {
    let start = Instant::now();
    let result = function();
    let elapsed = start.elapsed();
    println!("{} [ {:?} ] {}", name, elapsed, result);
}
