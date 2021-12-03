use std::collections::HashMap;
use std::num::ParseIntError;

use tools::utils::parse_string;

pub fn solve_second_problem(s: &str) -> Result<isize, ParseIntError> {
    let mut map = HashMap::new();

    for line in s.lines().into_iter() {
        let (key, value) = line.split_once(" ").expect("Error split");
        let value = parse_string(value)?;

        match key {
            "forward" => {
                *map.entry(key).or_insert(0) += value;
                *map.entry("depth").or_insert(0) += value * map.get("aim").unwrap_or(&0);
            }
            "up" => {
                *map.entry("aim").or_insert(0) -= value;
            }
            "down" => {
                *map.entry("aim").or_insert(0) += value;
            }
            _ => (),
        }
    }

    let result = map["forward"] * map["depth"];

    Ok(result as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let data =
            "forward 2\ndown 4\ndown 3\nup 4\ndown 1\ndown 8\nup 9\nforward 1\ndown 9\nforward 6";
        let result = solve_second_problem(data).unwrap();

        assert_eq!(result, 675_isize);
    }

    #[test]
    fn solve_second_problem_fails_parsing() {
        let data =
            "forward a\ndown 4\ndown 3\nup 4\ndown 1\ndown 8\nup 9\nforward 1\ndown 9\nforward 6";
        let result = solve_second_problem(data);

        assert!(result.is_err());
    }

    #[test]
    fn solve_second_problem_panics_split() {
        let data =
            "forward1\ndown 4\ndown 3\nup 4\ndown 1\ndown 8\nup 9\nforward 1\ndown 9\nforward 6";
        let result = std::panic::catch_unwind(|| solve_second_problem(data));

        assert!(result.is_err());
    }
}
