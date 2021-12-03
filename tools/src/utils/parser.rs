use std::num::ParseIntError;

pub fn parse_string_to_int_vector(s: String) -> Vec<i32> {
    s.lines()
        .into_iter()
        .map(|s| match parse_string(s) {
            Ok(n) => n,
            Err(_) => panic!("Cannot parse some element"),
        })
        .collect()
}

pub fn parse_string(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

#[cfg(test)]
mod tests {F
    use super::*;

    #[test]
    fn parse_string_vector_works_fine_with_valid_data() {
        let s = "1\n2\n3".to_string();
        let numbers = parse_string_to_int_vector(s);

        assert_eq!(numbers, [1, 2, 3]);
    }

    #[test]
    fn parse_string_vector_panics_with_invalid_data() {
        let s = "a\n1\n2\nb".to_string();
        let result = std::panic::catch_unwind(|| parse_string_to_int_vector(s));

        assert!(result.is_err());
    }

    #[test]
    fn parse_string_works_fine_with_valid_data() {
        let s = "12";
        let number = parse_string(s).unwrap();

        assert_eq!(number, 12);
    }

    #[test]
    fn parse_string_error_with_invalid_data() {
        let s = "a";
        let result = parse_string(s);

        assert!(result.is_err());
    }
}
