pub fn solve_second_problem(data: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let data = [1, 1, 1, 2, 3, 1, 1];
        let increased_times = solve_second_problem(&data);

        assert_eq!(increased_times, 2);
    }
}
