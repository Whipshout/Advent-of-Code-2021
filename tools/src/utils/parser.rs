pub fn parse_string_to_int_vector(s: String) -> Vec<i32> {
    s.lines()
        .into_iter()
        .map(|s| s.parse::<i32>().expect("Error parsing value"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_works_fine_with_valid_data() {
        let s = "1\n2\n3".to_string();
        let numbers = parse_string_to_int_vector(s);

        assert_eq!(numbers, [1, 2, 3]);
    }

    #[test]
    fn parse_string_panics_with_invalid_data() {
        let s = "a\n1\n2\nb".to_string();
        let result = std::panic::catch_unwind(|| parse_string_to_int_vector(s));

        assert!(result.is_err());
    }
}
