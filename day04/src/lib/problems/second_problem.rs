use crate::problems::bingo::Bingo;

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

pub fn solve_second_problem(input: &str) -> u32 {
    let (nums, boards) = input.split_once("\n\r").unwrap();

    let mut bingo = Bingo::new(nums, boards);

    let (board, mark, num) = bingo
        .numbers
        .into_iter()
        .filter_map(|n| {
            bingo
                .boards
                .drain_filter(|(b, m)| {
                    b.get(&n)
                        .map(|i| *m |= 1 << *i)
                        .map(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
                        .unwrap_or(false)
                })
                .map(|(b, m)| (b, m, n))
                .next()
        })
        .last()
        .unwrap();

    Bingo::calculate_winner(board, mark, num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_second_problem_works() {
        let data = include_str!("input_test.txt");
        let result = solve_second_problem(data);

        assert_eq!(result, 12035);
    }
}
