use std::error::Error;

use day03::run;

// ------------- First problem => Result: 3813416 --------------
// ------------- Second problem => Result: -------------
fn main() -> Result<(), Box<dyn Error>> {
    let (first_problem, second_problem) = match run("day03/input.txt") {
        Ok((first, second)) => (first, second),
        Err(e) => return Err(format!("Error: {}", e).into()),
    };

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);

    Ok(())
}
