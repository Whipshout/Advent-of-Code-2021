use std::error::Error;

use day02::run;

// ------------- First problem => Result: 1938402 --------------
// ------------- Second problem => Result: 1947878632 -------------
fn main() -> Result<(), Box<dyn Error>> {
    let (first_problem, second_problem) = match run("day02/input.txt") {
        Ok((first, second)) => (first, second),
        Err(e) => return Err(format!("Error: {}", e).into()),
    };

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);

    Ok(())
}
