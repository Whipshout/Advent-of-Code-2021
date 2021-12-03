use tools::utils::parse_string;

pub fn solve_first_problem(data: &Vec<Vec<char>>) -> isize {
    let mut numbers = vec![];

    let mut count = 0;

    for n in 0..12 {
        count = 0;

        for d in data {
            count += d[n].to_string().parse::<i32>().unwrap();
        }

        if count > data.len() as i32 / 2 {
            numbers.push("1");
        } else {
            numbers.push("0");
        }
    }

    let gamma = isize::from_str_radix(&numbers.join("").as_str(), 2).unwrap();

    for mut n in &mut numbers {
        if n == &"0" {
            *n = "1"
        } else {
            *n = "0"
        }
    }

    let epsilon = isize::from_str_radix(numbers.join("").as_str(), 2).unwrap();

    let result = gamma * epsilon;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_problem_works() {
        assert!(true);
    }
}
