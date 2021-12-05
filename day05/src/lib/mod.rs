#![feature(drain_filter)]

use problems::{solve_first_problem, solve_second_problem};
use tools::io::read_file;

mod problems;

pub fn run(path: &str) -> (usize, usize) {
    let input_file = read_file(path).unwrap();

    let result_first_problem = solve_first_problem(input_file.clone());
    let result_second_problem = solve_second_problem(input_file);

    (result_first_problem, result_second_problem)
}
