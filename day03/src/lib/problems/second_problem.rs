use tools::utils::parse_binary;

const WIDTH: usize = 12;

pub fn solve_second_problem(data: String) -> usize {
    let nums = data
        .lines()
        .map(|line| parse_binary(line).unwrap())
        .collect::<Vec<_>>();

    let oxygen = (0..WIDTH)
        .rev()
        .scan(nums.clone(), |oxygene, i| {
            let one = oxygene.iter().filter(|n| *n & 1 << i > 0).count() >= (oxygene.len() + 1) / 2;
            oxygene.drain_filter(|n| (*n & 1 << i > 0) != one);
            oxygene.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..WIDTH)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);
            co2.first().copied()
        })
        .last()
        .unwrap();

    (oxygen * co2) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let data = "011010101110\n111010100100".to_string();

        let result = solve_second_problem(data);

        assert_eq!(result, 6409080);
    }
}
