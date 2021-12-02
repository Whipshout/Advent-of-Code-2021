use day02::run;

// ------------- First problem => Result: 1754 --------------
// ------------- Second problem => Result: 1789 -------------
fn main() {
    let (first_problem, second_problem) = run("day02/input.txt").expect("Something went wrong");

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);
}
