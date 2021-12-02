pub fn solve_first_problem(data: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let data = [1, 2, 3, 2, 1];
        let increased_times = solve_first_problem(&data);

        assert_eq!(increased_times, 2);
    }
}
