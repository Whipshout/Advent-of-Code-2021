use std::collections::HashMap;
use std::num::ParseIntError;

use tools::utils::parse_string;

pub fn solve_first_problem(s: &str) -> Result<i32, ParseIntError> {
    let mut map = HashMap::new();

    for line in s.lines().into_iter() {
        let (key, value) = line.trim().split_once(" ").expect("Error split");

        *map.entry(key).or_insert(0) += parse_string(value)?;
    }

    let result = map["forward"] * (map["down"] - map["up"]);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let data =
            "forward 2\ndown 4\ndown 3\nup 4\ndown 1\ndown 8\nup 9\nforward 1\ndown 9\nforward 6";
        let result = solve_first_problem(data).unwrap();

        assert_eq!(result, 108);
    }

    #[test]
    fn solve_first_problem_fails_parsing() {
        let data =
            "forward a\ndown 4\ndown 3\nup 4\ndown 1\ndown 8\nup 9\nforward 1\ndown 9\nforward 6";
        let result = solve_first_problem(data);

        assert!(result.is_err());
    }

    #[test]
    fn solve_first_problem_panics_split() {
        let data =
            "forward1\ndown 4\ndown 3\nup 4\ndown 1\ndown 8\nup 9\nforward 1\ndown 9\nforward 6";
        let result = std::panic::catch_unwind(|| solve_first_problem(data));

        assert!(result.is_err());
    }
}
