pub fn parse_string_to_int_vector(s: String) -> Vec<i32> {
    s.lines()
        .into_iter()
        .map(|s| s.parse::<i32>().expect("Error parsing value"))
        .collect()
}
