use std::cmp::Ordering;

use tools::utils::{parse_binary, parse_string};

pub fn solve_first_problem(data: Vec<&Vec<char>>) -> isize {
    let mut numbers = vec![];

    for n in 0..12 {
        let mut count = 0;

        for d in data.iter() {
            count += parse_string(d[n].to_string()).unwrap();
        }

        match count.cmp(&(data.len() as i32 / 2)) {
            Ordering::Less => numbers.push("0"),
            Ordering::Equal => (),
            Ordering::Greater => numbers.push("1"),
        }
    }

    let gamma = parse_binary(numbers.join("")).unwrap();
    let epsilon = !gamma & ((1 << 12) - 1);

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use tools::utils::parse_string_to_char_vector;

    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let input = "000000000000\n111111111111\n111111111111";
        let data = parse_string_to_char_vector(input.to_string());

        let result = solve_first_problem(data.iter().collect());

        assert_eq!(result, 0);
    }
}
