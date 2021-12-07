use day06::run;

// ------------- First problem => Result: 359344 ----------------------
// ------------- Second problem => Result: 1629570219571 --------------
fn main() {
    let input = include_str!("input.txt");
    let (first_problem, second_problem) = run(input);

    println!("First problem result: {}", first_problem);
    println!("Second problem result: {}", second_problem);
}
