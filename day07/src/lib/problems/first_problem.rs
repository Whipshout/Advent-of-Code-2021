use std::str::FromStr;

pub fn solve_first_problem(input: &str) -> i32 {
    let positions: Vec<_> = input
        .split(',')
        .into_iter()
        .map(|n| i32::from_str(n).unwrap())
        .collect();

    let max = positions.iter().max().unwrap();

    let mut temp_fuel = 2000000000;

    (0..=*max).for_each(|n| {
        let fuel = calculate_fuel(positions.clone(), n);

        if fuel < temp_fuel {
            temp_fuel = fuel;
        }
    });

    temp_fuel
}

fn calculate_fuel(positions: Vec<i32>, n: i32) -> i32 {
    positions
        .into_iter()
        .fold(0, |sum, val| sum + ((val - n).abs()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let input = "1101,1,29,67,1102,0,1,65,1008,65";
        let result = solve_first_problem(input);

        assert_eq!(result, 3247);
    }
}
