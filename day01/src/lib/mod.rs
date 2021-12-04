use problems::{solve_first_problem, solve_second_problem};
use tools::io::read_file;
use tools::utils::parse_string_to_int_vector;

mod problems;

pub fn run(path: &str) -> (i32, i32) {
    let input_file = read_file(path).unwrap();
    let data: Vec<i32> = parse_string_to_int_vector(input_file);

    let result_first_problem = solve_first_problem(&data);
    let result_second_problem = solve_second_problem(&data);

    (result_first_problem, result_second_problem)
}
