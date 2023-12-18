mod day_01;
mod day_02;

use std::time::Instant;

fn main() {
    run_measure("Day 1.1", &day_01::part_1);
    run_measure("Day 1.2", &day_01::part_2);
    run_measure("Day 2.1", &day_02::part_1);
}

fn run_measure<T: std::fmt::Display>(name: &str, function: &dyn Fn() -> T) {
    let start = Instant::now();
    let result = function();
    let elapsed = start.elapsed();
    println!("{} [ {:?} ] {}", name, elapsed, result);
}
