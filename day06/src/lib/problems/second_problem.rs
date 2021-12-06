pub fn solve_second_problem(data: String) -> usize {
    let mut map = data.split(',').fold([0; 9], |mut map, n| {
        map[n.parse::<usize>().unwrap()] += 1;
        map
    });

    (1..256).for_each(|day| map[(day + 7) % 9] += map[day % 9]);

    map.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let input = "4,3,4,5,2,1,1,5,5,3,3,1,5,1,4,2,2,3,1,5,1,4,1,2".to_string();
        let result = solve_second_problem(input);

        assert_eq!(result, 127530560332);
    }
}
