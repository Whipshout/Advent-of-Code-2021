use std::collections::HashMap;
use std::error::Error;

use tools::utils::parse_string;

pub fn solve_first_problem(s: &str) -> Result<i32, Box<dyn Error>> {
    let mut map = HashMap::new();

    for line in s.lines().into_iter() {
        let (key, value) = match line.trim().split_once(" ") {
            Some((key, value)) => (key, value),
            None => return Err("Cannot split some line".into()),
        };

        *map.entry(key).or_insert(0) += parse_string(value)?;
    }

    match (map.get("forward"), map.get("down"), map.get("up")) {
        (Some(forward), Some(down), Some(up)) => Ok(forward * (down - up)),
        _ => Err("Some keys not found in hash map".into()),
    }
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
    fn solve_first_problem_error_split() {
        let data =
            "forward1\ndown 4\ndown 3\nup 4\ndown 1\ndown 8\nup 9\nforward 1\ndown 9\nforward 6";
        let result = solve_first_problem(data);

        assert!(result.is_err());
    }

    #[test]
    fn solve_first_problem_error_keys_not_found() {
        let data = "asdf 1\ndown 4\nqwer 3\nasdf 4\nqwer 1\nasdf 8";
        let result = solve_first_problem(data);

        assert!(result.is_err());
    }
}
