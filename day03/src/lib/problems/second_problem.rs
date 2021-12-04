use tools::utils::{parse_binary, parse_string};

pub fn solve_second_problem(data: Vec<&Vec<char>>) -> isize {
    check_data(data.clone(), data.clone(), 0)
}

fn check_data(oxygen_data: Vec<&Vec<char>>, co2_data: Vec<&Vec<char>>, i: i32) -> isize {
    let mut count_oxygen = 0;
    let mut count_co2 = 0;
    let mut oxygen_vector = vec![];
    let mut co2_vector = vec![];

    for d in oxygen_data.iter() {
        count_oxygen += parse_string(d[i as usize].to_string()).unwrap();
    }

    for &data in oxygen_data.iter() {
        if count_oxygen >= oxygen_data.len() as i32 / 2 {
            if data[i as usize] == '1' {
                oxygen_vector.push(data);
            }
        } else if data[i as usize] == '0' {
            oxygen_vector.push(data);
        }
    }

    for d in co2_data.iter() {
        count_co2 += parse_string(d[i as usize].to_string()).unwrap();
    }

    for &data in co2_data.iter() {
        if count_co2 >= co2_data.len() as i32 / 2 {
            if data[i as usize] == '0' {
                co2_vector.push(data);
            }
        } else if data[i as usize] == '1' {
            co2_vector.push(data);
        }
    }

    if oxygen_vector.len() == 1 && co2_vector.len() == 1 {
        parse_binary(oxygen_vector[0].iter().collect::<String>()).unwrap()
            * parse_binary(co2_vector[0].iter().collect::<String>()).unwrap()
    } else {
        check_data(oxygen_vector, co2_vector, i + 1)
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
