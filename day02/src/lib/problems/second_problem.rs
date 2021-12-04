use tools::utils::parse_string;

pub fn solve_second_problem(s: &str) -> isize {
    let (forward, depth, _) = s.lines().map(|line| line.split_once(" ").unwrap()).fold(
        (0, 0, 0),
        |(forward, depth, aim), (key, value)| match (key, parse_string(value).unwrap()) {
            ("forward", value) => (forward + value, depth + aim * value, aim),
            ("down", value) => (forward, depth, aim + value),
            ("up", value) => (forward, depth, aim - value),
            _ => unreachable!(),
        },
    );

    (forward * depth) as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let data =
            "forward 2\ndown 4\ndown 3\nup 4\ndown 1\ndown 8\nup 9\nforward 1\ndown 9\nforward 6";
        let result = solve_second_problem(data);

        assert_eq!(result, 675_isize);
    }
}
