use problems::{solve_first_problem, solve_second_problem};
use tools::io::read_file;
use tools::utils::parse_string_to_char_vector;

mod problems;

pub fn run(path: &str) -> (isize, isize) {
    let input_file = read_file(path).unwrap();
    let data = parse_string_to_char_vector(input_file);

    let result_first_problem = solve_first_problem(data.iter().collect());
    let result_second_problem = solve_second_problem(data.iter().collect());

    (result_first_problem, result_second_problem)
}
