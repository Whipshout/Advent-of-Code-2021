use tools::utils::parse_string;

pub fn solve_first_problem(s: &str) -> i32 {
    let (forward, depth) = s.lines().map(|line| line.split_once(" ").unwrap()).fold(
        (0, 0),
        |(forward, depth), (key, value)| match (key, parse_string(value).unwrap()) {
            ("forward", value) => (forward + value, depth),
            ("down", value) => (forward, depth + value),
            ("up", value) => (forward, depth - value),
            _ => unreachable!(),
        },
    );

    forward * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let data =
            "forward 2\ndown 4\ndown 3\nup 4\ndown 1\ndown 8\nup 9\nforward 1\ndown 9\nforward 6";
        let result = solve_first_problem(data);

        assert_eq!(result, 108);
    }
}
