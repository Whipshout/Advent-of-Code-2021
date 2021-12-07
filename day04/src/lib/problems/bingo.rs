use std::collections::HashMap;

pub struct Bingo {
    pub numbers: Vec<u8>,
    pub boards: Vec<(HashMap<u8, usize>, u32)>,
}

impl Bingo {
    pub fn new(numbers: &str, boards: &str) -> Self {
        Self {
            numbers: Self::generate_numbers(numbers),
            boards: Self::generate_boards(boards),
        }
    }

    fn generate_numbers(numbers: &str) -> Vec<u8> {
        numbers
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect()
    }

    pub fn generate_boards(boards: &str) -> Vec<(HashMap<u8, usize>, u32)> {
        boards
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
            .collect()
    }

    pub fn calculate_winner(board: HashMap<u8, usize>, mark: u32, num: u8) -> u32 {
        board
            .into_iter()
            .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * num as u32)
            .sum::<u32>()
    }
}
