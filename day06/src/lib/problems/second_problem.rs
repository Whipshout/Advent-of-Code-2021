use crate::problems::lanternfish::Pool;

pub fn solve_second_problem(input: &str) -> isize {
    let mut pool = Pool::create_pool(input);

    pool.complete_cycles(256);

    pool.total_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let input = "4,3,4,5,2,1,1,5,5,3,3,1,5,1,4,2,2,3,1,5,1,4,1,2";
        let result = solve_second_problem(input);

        assert_eq!(result, 127530560332);
    }
}
