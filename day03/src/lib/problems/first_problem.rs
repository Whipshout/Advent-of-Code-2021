use tools::utils::parse_binary;

const WIDTH: usize = 12;
const COUNT: usize = 1000;

pub fn solve_first_problem(input: &str) -> usize {
    let gamma = input
        .lines()
        .map(|line| parse_binary(line).unwrap())
        .fold(vec![0; WIDTH], |count, bits| {
            count
                .into_iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        })
        .into_iter()
        .enumerate()
        .map(|(i, b)| ((b >= COUNT / 2) as u32) << i)
        .sum::<u32>();
    let epsilon = !gamma & ((1 << 12) - 1);

    (gamma * epsilon) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let data = "000000000000\n111111111111\n111111111111";

        let result = solve_first_problem(data);

        assert_eq!(result, 0);
    }
}
