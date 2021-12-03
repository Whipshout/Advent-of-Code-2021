use std::collections::HashMap;
use std::error::Error;

use tools::utils::parse_string;

pub fn solve_second_problem(s: &str) -> Result<isize, Box<dyn Error>> {
    let mut map = HashMap::new();

    for line in s.lines().into_iter() {
        let (key, value) = match line.split_once(" ") {
            Some((key, value)) => (key, value),
            None => panic!("Cannot split lines"),
        };
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

    match (map.get("forward"), map.get("depth")) {
        (Some(forward), Some(depth)) => Ok((forward * depth) as isize),
        _ => panic!("Keys not found in HashMap"),
    }
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

    #[test]
    fn solve_second_problem_panics_keys_not_found() {
        let data = "asdf 1\ndown 4\nqwer 3\nasdf 4\nqwer 1\nasdf 8";
        let result = std::panic::catch_unwind(|| solve_second_problem(data));

        assert!(result.is_err());
    }
}
