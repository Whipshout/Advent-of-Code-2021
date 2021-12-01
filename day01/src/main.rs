use day01::{solve_first_problem, solve_second_problem};
use tools::io::read_file;
use tools::utils::parse_string_to_int_vector;

// ------------- First problem => Result: 1754 --------------
// ------------- Second problem => Result: 1789 -------------
fn main() {
    let input_file = read_file("day01/input.txt").expect("Error reading file");
    let data: Vec<i32> = parse_string_to_int_vector(input_file);

    let increased_times_first_problem = solve_first_problem(&data);
    let increased_times_second_problem = solve_second_problem(&data);

    println!("First problem result: {}", increased_times_first_problem);
    println!("Second problem result: {}", increased_times_second_problem);
}
