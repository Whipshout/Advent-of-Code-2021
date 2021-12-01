use std::fs::File;
use std::io::Read;

fn main() {
    // ------------------------ Get input -------------------------
    let input_file = read_file("input.txt").expect("Cannot read file");
    let data: Vec<i32> = input_file
        .lines()
        .into_iter()
        .map(|s| s.parse::<i32>().expect("Error parsing value"))
        .collect();
    // ------------------------ Get input -------------------------

    // ------------- First problem => Solution: 1754 --------------
    let mut data_peekable = data.iter().peekable();
    let mut increased_times = 0;

    while let Some(element) = data_peekable.next() {
        if let Some(next_element) = data_peekable.peek() {
            if next_element > &element {
                increased_times += 1;
            }
        }
    }
    // ------------- First problem => Solution: 1754 --------------

    // ------------- Second problem => Solution: 1789 -------------
    let mut data_windows = data.windows(3);
    let mut previous_input: i32 = data_windows.next().expect("Error").iter().sum();
    let mut increased_times2 = 0;

    data_windows.skip(1).for_each(|vector| {
        let sum: i32 = vector.iter().sum();
        if sum > previous_input {
            increased_times2 += 1;
        }
        previous_input = sum;
    });
    // ------------- Second problem => Solution: 1789 -------------

    println!("First problem solution: {}", increased_times);
    println!("Second problem solution: {}", increased_times2);
}

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(&path)?;
    let file_len = file.metadata().unwrap().len();
    let mut file_contents = Vec::with_capacity(file_len as usize + 1);
    file.read_to_end(&mut file_contents)?;

    Ok(unsafe { String::from_utf8_unchecked(file_contents) })
}
