use std::collections::HashMap;
use crate::shared::utils::{lines};
use std::fs;

#[derive(Debug)]
struct BingoBoard {
    board: [u8; 25],
    map: HashMap<u8, usize>,
}

const MARKED: u8 = 255;

impl BingoBoard {
    fn new<I>(board: I) -> Self
    where
        I: IntoIterator<Item = u8>
    {
        let mut array = [MARKED; 25];
        let mut map = HashMap::with_capacity(25);
        for (i, num) in board.into_iter().take(25).enumerate() {
            array[i] = num;
            map.insert(num, i);
        }
        BingoBoard{ board: array, map }
    }

    fn get(self: &Self, row: usize, col: usize) -> u8 {
        self.board[(row - 1) * 5 + col - 1]
    }

    fn total(self: &Self) -> u16 {
        self.board.into_iter().filter(|x| *x != MARKED).map(|x| x as u16).sum()
    }

    fn mark(self: &mut Self, play: u8) -> Option<(usize, usize)> {
        let played = {
            self.map.remove(&play)
        };

        match played {
            Some(i) => {
                self.board[i] = MARKED;
                Some((i / 5, i % 5 + 1))
            },
            None => None
        }
    }

    fn bingo(self: &Self) -> bool {
        for row in 1..5 {
            if self.get(row, 1) == MARKED &&
                self.get(row, 2) == MARKED &&
                self.get(row, 3) == MARKED &&
                self.get(row, 4) == MARKED &&
                self.get(row, 5) == MARKED {
                return true;
            }
        }

        for col in 1..5 {
            if self.get(1, col) == MARKED &&
                self.get(2, col) == MARKED &&
                self.get(3, col) == MARKED &&
                self.get(4, col) == MARKED &&
                self.get(5, col) == MARKED {
                return true;
            }
        }

        false
    }
}

fn process(input: &str) {
    let lines = lines(&input);

    let mut boards: Vec<BingoBoard> = Vec::new();
    for i in (1..lines.len()).step_by(5) {
        let board: Vec<u8> = lines[i..i+5]
            .join(" ")
            .split(' ')
            .filter(|x| x.trim() != "")
            .map(|x| x.trim().parse::<u8>().unwrap())
            .collect();
        boards.push(BingoBoard::new(board));
    }

    let plays: Vec<u8> = lines[0].split(',').map(|x| x.parse::<u8>().unwrap()).collect();
    for play in plays {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.bingo() {
                continue
            }

            let result = board.mark(play);
            if result.is_some() && board.bingo() {
                println!("Winner {} - {} {}", i, play, board.total() * play as u16);
            }
        }
    }
}

pub fn run() {
    let test_input = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    println!("Test Input:");
    process(test_input);

    let input = fs::read_to_string("data/day4.txt").expect("Failed to read the file.");
    println!("Real Input:");
    process(&input);
}

