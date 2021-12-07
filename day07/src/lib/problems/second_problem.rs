use std::str::FromStr;

use crate::problems::crabs::Crabs;

pub fn solve_second_problem(input: &str) -> i32 {
    let positions: Vec<_> = input
        .split(',')
        .into_iter()
        .map(|n| i32::from_str(n).unwrap())
        .collect();

    let mut crabs = Crabs::new(positions);
    let max = crabs.max_position();

    crabs.calculate_lowest_fuel(max, true);

    crabs.lowest_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let input = "1101,1,29,67,1102,0,1,65,1008,65";
        let result = solve_second_problem(input);

        assert_eq!(result, 1139069);
    }
}
