use day04::run;

// ------------- First problem => Result: 39984 --------------
// ------------- Second problem => Result: 8468 --------------
fn main() {
    let input = include_str!("input.txt");
    let (first_problem, second_problem) = run(input);

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);
}
