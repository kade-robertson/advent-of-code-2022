use std::{env, time::Instant};

use crate::problem::Problem;

#[macro_use]
mod macros;
mod problem;
mod problem_01;
mod problem_02;
mod problem_03;
mod util;

fn main() {
    println!("~ Advent of Code 2022 ~");

    let censor_results = env::args().any(|x| x == *"--censor");
    let do_censoring = |result: i64| {
        if censor_results {
            "censored".to_owned()
        } else {
            result.to_string()
        }
    };

    let problems: Vec<Box<dyn Problem>> = vec![
        Box::new(problem_01::Problem01::new()),
        Box::new(problem_02::Problem02::new()),
        Box::new(problem_03::Problem03::new()),
    ];
    let mut duration = Instant::now().elapsed();
    problems.iter().for_each(|problem| {
        println!("{}", problem.name());

        print!(" - Part 1: ");
        let part1_start = Instant::now();
        let part1_result = problem.solve();
        let part1_duration = part1_start.elapsed();
        duration += part1_duration;
        println!(
            "{} (took {:.2?})",
            do_censoring(part1_result),
            part1_duration
        );

        print!(" - Part 2: ");
        let part2_start = Instant::now();
        let part2_result = problem.solve_part2();
        let part2_duration = part2_start.elapsed();
        duration += part2_duration;
        println!(
            "{} (took {:.2?})",
            do_censoring(part2_result),
            part2_duration
        );
    });
    println!("Took a total of {:.2?}", duration);
}
