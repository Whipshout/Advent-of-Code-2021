use crate::problems::bingo::Bingo;

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

pub fn solve_first_problem(input: &str) -> u32 {
    let (nums, boards) = input.split_once("\n\r").unwrap();

    let mut bingo = Bingo::new(nums, boards);

    let (board, mark, num) = bingo
        .numbers
        .into_iter()
        .find_map(|n| {
            bingo.boards.iter_mut().find_map(|(b, m)| {
                b.get(&n)
                    .map(|i| *m |= 1 << *i)
                    .filter(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
                    .map(|_| (b.clone(), *m, n))
            })
        })
        .unwrap();

    Bingo::calculate_winner(board, mark, num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let data = include_str!("input_test.txt");
        let result = solve_first_problem(data);

        assert_eq!(result, 18104);
    }
}
