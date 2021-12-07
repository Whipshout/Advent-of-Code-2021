const WIDTH: usize = 12;

pub fn calculate_value(numbers: Vec<usize>, is_equal: bool) -> usize {
    (0..WIDTH)
        .rev()
        .scan(numbers, |value, i| {
            let one = value.iter().filter(|n| *n & 1 << i > 0).count() >= (value.len() + 1) / 2;
            value.drain_filter(|n| match is_equal {
                true => (*n & 1 << i > 0) == one,
                false => (*n & 1 << i > 0) != one,
            });
            value.first().copied()
        })
        .last()
        .unwrap()
}
