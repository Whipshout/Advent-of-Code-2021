use crate::problems::submarine::{Direction, Submarine};

pub fn solve_first_problem(input: &str) -> i32 {
    let input: Vec<_> = input
        .lines()
        .map(|line| line.parse::<Direction>())
        .collect();

    let mut submarine = Submarine::default();

    for direction in input {
        submarine.move_sub(direction.unwrap());
    }

    submarine.position()
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
