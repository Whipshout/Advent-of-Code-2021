use day03::run;

// ------------- First problem => Result: 3813416 --------------
// ------------- Second problem => Result: 2990784 -------------
fn main() {
    let input = include_str!("input.txt");
    let (first_problem, second_problem) = run(input);

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);
}
