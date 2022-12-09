use std::{
    env,
    time::{Duration, Instant},
};

use crate::problem::{Problem, Solution};

#[macro_use]
mod macros;
mod problem;
mod problem_01;
mod problem_02;
mod problem_03;
mod problem_04;
mod problem_05;
mod problem_06;
mod util;

fn main() {
    println!("~ Advent of Code 2022 ~");

    let censor_results = env::args().any(|x| x == *"--censor");
    let do_benchmarking = env::args().any(|x| x == *"--bench");
    let iterations = if do_benchmarking { 10 } else { 1 };

    let do_censoring = |result: Solution| {
        if censor_results {
            "censored".to_owned()
        } else {
            match result {
                Solution::U64(v) => v.to_string(),
                Solution::U32(v) => v.to_string(),
                Solution::U16(v) => v.to_string(),
                Solution::Str(v) => v,
            }
        }
    };

    let problems: Vec<Box<dyn Problem>> = vec![
        Box::new(problem_01::Problem01::new()),
        Box::new(problem_02::Problem02::new()),
        Box::new(problem_03::Problem03::new()),
        Box::new(problem_04::Problem04::new()),
        Box::new(problem_05::Problem05::new()),
        Box::new(problem_06::Problem06::new()),
    ];
    let mut duration = Instant::now().elapsed();
    problems.iter().for_each(|problem| {
        println!("{}", problem.name());

        let mut part1_total = Duration::ZERO;
        let mut part1_result = Solution::Str("N/A".to_string());

        print!(" - Part 1: ");
        for _ in 0..iterations {
            let part1_start = Instant::now();
            part1_result = problem.solve();
            let part1_duration = part1_start.elapsed();
            duration += part1_duration;
            part1_total += part1_duration;
        }
        println!(
            "{} (took {:.2?})",
            do_censoring(part1_result),
            part1_total / iterations
        );

        let mut part2_total = Duration::ZERO;
        let mut part2_result = Solution::Str("N/A".to_string());

        print!(" - Part 2: ");
        for _ in 0..iterations {
            let part2_start = Instant::now();
            part2_result = problem.solve_part2();
            let part2_duration = part2_start.elapsed();
            duration += part2_duration;
            part2_total += part2_duration;
        }
        println!(
            "{} (took {:.2?})",
            do_censoring(part2_result),
            part2_total / iterations
        );
    });
    println!("Took a total of {:.2?}", duration / iterations);
}
