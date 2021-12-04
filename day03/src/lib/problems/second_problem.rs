use tools::utils::parse_binary;

pub fn solve_second_problem(data: Vec<&Vec<char>>) -> isize {
    let oxygen = check_oxygen(data.clone(), 0);
    let co2 = check_co2(data.clone(), 0);

    oxygen * co2
}

fn check_oxygen(data: Vec<&Vec<char>>, i: i32) -> isize {
    let mut count = 0;
    let mut temp_vector = vec![];

    for d in data.iter() {
        count += d[i as usize].to_string().parse::<i32>().unwrap();
    }

    data.iter().for_each(|&x| {
        if count >= data.len() as i32 / 2 {
            if x[i as usize] == '1' {
                temp_vector.push(x);
            }
        } else if x[i as usize] == '0' {
            temp_vector.push(x);
        }
    });

    if temp_vector.len() == 1 {
        parse_binary(temp_vector[0].iter().collect::<String>().as_str()).unwrap()
    } else {
        check_oxygen(temp_vector, i + 1)
    }
}

fn check_co2(data: Vec<&Vec<char>>, i: i32) -> isize {
    let mut count = 0;
    let mut temp_vector = vec![];

    for d in data.iter() {
        count += d[i as usize].to_string().parse::<i32>().unwrap();
    }

    data.iter().for_each(|&x| {
        if count >= data.len() as i32 / 2 {
            if x[i as usize] == '0' {
                temp_vector.push(x);
            }
        } else if x[i as usize] == '1' {
            temp_vector.push(x);
        }
    });

    if temp_vector.len() == 1 {
        parse_binary(temp_vector[0].iter().collect::<String>().as_str()).unwrap()
    } else {
        check_co2(temp_vector, i + 1)
    }
}

#[cfg(test)]
mod tests {
    use tools::utils::parse_string_to_char_vector;

    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let input = "011010101110\n111010100100";
        let data = parse_string_to_char_vector(input.to_string());

        let result = solve_second_problem(data.iter().collect());

        assert_eq!(result, 6409080);
    }
}
