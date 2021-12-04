use day01::run;
use std::time::Instant;

// ------------- First problem => Result: 1754 --------------
// ------------- Second problem => Result: 1789 -------------
fn main() {
    let now = Instant::now();
    let (first_problem, second_problem) = run("day01/input.txt");
    let elapsed = now.elapsed();

    println!("{}", elapsed.as_nanos());
    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);
}
