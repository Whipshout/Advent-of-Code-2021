use day07::run;

// ------------- First problem => Result: 355592 -----------------
// ------------- Second problem => Result: 101618069 -------------
fn main() {
    let input = include_str!("input.txt");
    let (first_problem, second_problem) = run(input);

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);
}
