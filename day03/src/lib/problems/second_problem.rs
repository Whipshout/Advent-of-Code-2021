use crate::problems::binary::calculate_value;
use tools::utils::parse_binary;

pub fn solve_second_problem(input: &str) -> usize {
    let data = input
        .lines()
        .map(|line| parse_binary(line).unwrap())
        .collect::<Vec<_>>();

    let oxygen = calculate_value(data.clone(), false);
    let co2 = calculate_value(data, true);

    (oxygen * co2) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let data = "011010101110\n111010100100";

        let result = solve_second_problem(data);

        assert_eq!(result, 6409080);
    }
}
