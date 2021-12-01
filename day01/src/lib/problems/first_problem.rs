pub fn solve_first_problem(data: &[i32]) -> i32 {
    let mut data_peekable = data.iter().peekable();
    let mut increased_times = 0;

    while let Some(element) = data_peekable.next() {
        if let Some(next_element) = data_peekable.peek() {
            if next_element > &element {
                increased_times += 1;
            }
        }
    }

    increased_times
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
