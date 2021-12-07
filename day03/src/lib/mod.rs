#![feature(drain_filter)]

use problems::{solve_first_problem, solve_second_problem};

mod problems;

pub fn run(input: &str) -> (usize, usize) {
    let result_first_problem = solve_first_problem(input);
    let result_second_problem = solve_second_problem(input);

    (result_first_problem, result_second_problem)
}
