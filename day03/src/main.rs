use std::error::Error;

use day03::run;

// ------------- First problem => Result: 3813416 --------------
// ------------- Second problem => Result: 2990784 -------------
fn main() -> Result<(), Box<dyn Error>> {
    let (first_problem, second_problem) = run("day03/input.txt");

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);

    Ok(())
}
