use problems::{solve_first_problem, solve_second_problem};
use tools::io::read_file;
use tools::utils::parse_string_to_int_vector;

mod problems;

pub fn run(path: &str) -> Result<(i32, i32), std::io::Error> {
    let input_file = read_file(path)?;
    let data: Vec<i32> = parse_string_to_int_vector(input_file);

    let increased_times_first_problem = solve_first_problem(&data);
    let increased_times_second_problem = solve_second_problem(&data);

    Ok((
        increased_times_first_problem,
        increased_times_second_problem,
    ))
}
