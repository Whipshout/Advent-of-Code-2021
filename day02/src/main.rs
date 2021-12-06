use day02::run;

// ------------- First problem => Result: 1938402 --------------
// ------------- Second problem => Result: 1947878632 -------------
fn main() {
    let input = include_str!("input.txt");
    let (first_problem, second_problem) = run(input);

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);
}
