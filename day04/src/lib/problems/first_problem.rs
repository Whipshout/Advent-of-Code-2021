use std::collections::BTreeMap;

const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

pub fn solve_first_problem(data: String) -> u32 {
    let (nums, boards) = data.split_once("\n\r").unwrap();

    let mut boards: Vec<(BTreeMap<u8, usize>, u32)> = boards
        .split("\n\r")
        .map(|b| {
            (
                b.split_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.trim().parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    let (board, mark, num) = nums
        .split(',')
        .map(|n| n.parse().unwrap())
        .find_map(|n| {
            boards.iter_mut().find_map(|(b, m)| {
                b.get(&n)
                    .map(|i| *m |= 1 << *i)
                    .filter(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
                    .map(|_| (b.clone(), *m, n))
            })
        })
        .unwrap();

    board
        .into_iter()
        .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * num as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use tools::io::read_file;

    use super::*;

    #[test]
    fn solve_first_problem_works() {
        let data = read_file("input_test.txt").unwrap();
        let result = solve_first_problem(data);

        assert_eq!(result, 18104);
    }
}
