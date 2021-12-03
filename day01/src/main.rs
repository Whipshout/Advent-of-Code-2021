use std::error::Error;

use day01::run;

// ------------- First problem => Result: 1754 --------------
// ------------- Second problem => Result: 1789 -------------
fn main() -> Result<(), Box<dyn Error>> {
    let (first_problem, second_problem) = match run("day01/input.txt") {
        Ok((first, second)) => (first, second),
        Err(e) => return Err(format!("Error: {}", e).into()),
    };

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);

    Ok(())
}
