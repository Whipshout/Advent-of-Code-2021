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

pub fn solve_second_problem(data: &[i32]) -> i32 {
    let mut data_windows = data.windows(3).peekable();
    let mut increased_times = 0;

    while let Some(element) = data_windows.next() {
        let element: i32 = element.iter().sum();
        if let Some(next_element) = data_windows.peek() {
            let next_element: i32 = next_element.iter().sum();
            if next_element > element {
                increased_times += 1;
            }
        }
    }

    increased_times
}
