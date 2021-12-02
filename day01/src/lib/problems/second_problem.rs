pub fn solve_second_problem(data: &[i32]) -> i32 {
    let mut data_windows = data.windows(3).peekable();
    let mut increased_times = 0;

    while let Some(element) = data_windows.next() {
        if let Some(next_element) = data_windows.peek() {
            let element: i32 = element.iter().sum();
            let next_element: i32 = next_element.iter().sum();
            if next_element > element {
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
    fn solve_second_problem_works() {
        let data = [1, 1, 1, 2, 3, 1, 1];
        let increased_times = solve_second_problem(&data);

        assert_eq!(increased_times, 2);
    }
}
