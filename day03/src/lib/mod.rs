use std::error::Error;

use problems::{solve_first_problem, solve_second_problem};
use tools::io::read_file;
use tools::utils::parse_string_to_char_vector;

mod problems;

pub fn run(path: &str) -> Result<(isize, isize), Box<dyn Error>> {
    let input_file = read_file(path)?;
    let data = parse_string_to_char_vector(input_file);

    let increased_times_first_problem = solve_first_problem(&data);
    let increased_times_second_problem = solve_second_problem(&data);

    Ok((
        increased_times_first_problem,
        increased_times_second_problem,
    ))
}
