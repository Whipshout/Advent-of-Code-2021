use std::collections::HashMap;
use problems::{solve_first_problem, solve_second_problem};
use tools::io::read_file;
use tools::utils::parse_string_to_int_vector;

mod problems;

// First problem result: 1938402
// Second problem result: 1947878632
pub fn run(path: &str) -> Result<(i32, i32), std::io::Error> {
    let input_file = read_file(path)?;

    // let mut data = HashMap::new();

    // for line in input_file.lines().into_iter() {
    //     let (key, value) = line.trim().split_once(" ").unwrap();
    //
    //     *data.entry(key).or_insert(0) += value.parse::<i32>().unwrap();
    // }
    //
    // let result = data["forward"] * (data["down"] - data["up"]);
    //
    // println!("{}", result);

    let mut data = HashMap::new();

    for line in input_file.lines().into_iter() {
        let (key, value) = line.split_once(" ").unwrap();
        let value = value.parse::<isize>().unwrap();

        match key {
            "forward" => {
                *data.entry(key).or_insert(0) += value;
                *data.entry("depth").or_insert(0) += (value * data.get("aim").unwrap_or(&0));
            }
            "up" => {
                *data.entry("aim").or_insert(0) -= value;
            }
            "down" => {
                *data.entry("aim").or_insert(0) += value;
            }
            _ => ()
        }
    }

    let result = data["forward"] * data["depth"];

    println!("{}", result);

    Ok((
        0,
        0,
    ))
}
