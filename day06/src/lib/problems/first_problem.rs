use crate::problems::lanternfish::Pool;

pub fn solve_first_problem(input: &str) -> isize {
    let mut pool = Pool::create_pool(input);

    pool.complete_cycles(80);

    pool.total_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let input = "4,3,4,5,2,1,1,5,5,3,3,1,5,1,4,2,2,3,1,5,1,4,1,2";
        let result = solve_first_problem(input);

        assert_eq!(result, 28073);
    }
}
