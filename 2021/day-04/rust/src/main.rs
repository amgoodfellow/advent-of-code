#![allow(dead_code)]
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<(usize, bool)>>,
    board_contents: HashSet<usize>,
    marks: usize,
    winner: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: vec![vec![(0, false); 5]; 5],
            board_contents: HashSet::new(),
            marks: 0,
            winner: false,
        }
    }

    pub fn mark(&mut self, value: usize) -> bool {
        if self.board_contents.contains(&value) {
            for row_index in 0..self.board.len() {
                for column_index in 0..self.board[1].len() {
                    if self.board[row_index][column_index].0 == value {
                        self.marks += 1;
                        self.board[row_index][column_index].1 = true;
                        return true;
                    }
                }
            }
        }
        false
    }

    // This could be made more efficient with two arrays prob and a single n^2
    pub fn is_winner(&mut self) -> bool {
        if self.marks >= 5 {
            for row in &self.board {
                let mut row_count = 0;
                for element in row {
                    if element.1 {
                        row_count += 1;
                    }
                }
                if row_count == 5 {
                    self.winner = true;
                    return true;
                }
            }
            for column_index in 0..self.board[1].len() {
                let mut col_count = 0;
                for row_index in 0..self.board.len() {
                    if self.board[row_index][column_index].1 {
                        col_count += 1;
                    }
                }
                if col_count == 5 {
                    self.winner = true;
                    return true;
                }
            }
        }
        false
    }

    pub fn calculate_score(&self, number_called: usize) -> usize {
        let board_sum = self.board.iter().fold(0, |acc, row| {
            let mut sum = 0;
            for element in row {
                if !element.1 {
                    sum += element.0;
                }
            }
            acc + sum
        });

        board_sum * number_called
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> (Vec<usize>, Vec<Board>) {
    let mut draw_numbers = Vec::new();
    let mut boards: Vec<Board> = Vec::new();

    let mut current_board = Board::new();
    let mut current_board_row = 0;

    for (index, content) in BufReader::new(File::open(filename).expect("File not found"))
        .lines()
        .enumerate()
    {
        let content = content.unwrap();

        if index == 0 {
            draw_numbers = content
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
        } else if content == "" {
            if index != 1 {
                boards.push(current_board);
                current_board = Board::new();
                current_board_row = 0;
            }
        } else {
            let board_line = content
                .split(" ")
                .filter(|c| *c != "")
                .map(|x| {
                    let digit = x.parse::<usize>().unwrap();
                    current_board.board_contents.insert(digit);
                    (digit, false)
                })
                .collect();
            current_board.board[current_board_row] = board_line;
            current_board_row += 1;
        }
    }

    (draw_numbers, boards)
}

fn part_one(draw_numbers: Vec<usize>, mut boards: Vec<Board>) -> Option<usize> {
    for drawn_number in draw_numbers {
        for board in boards.iter_mut() {
            if board.mark(drawn_number) {
                if board.is_winner() {
                    return Some(board.calculate_score(drawn_number));
                }
            }
        }
    }
    None
}

fn part_two(draw_numbers: Vec<usize>, mut boards: Vec<Board>) -> Option<usize> {
    let mut won_count = 0;
    let board_count = boards.len();
    for drawn_number in draw_numbers {
        for board in boards.iter_mut() {
            if board.mark(drawn_number) {
                if !board.winner && board.is_winner() {
                    won_count += 1;
                    if won_count == board_count {
                        return Some(board.calculate_score(drawn_number));
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let (numbers, boards) = lines_from_file("../input/input.txt");
    println!("{:?}", part_one(numbers, boards));
    let (numbers, boards) = lines_from_file("../input/input.txt");
    println!("{:?}", part_two(numbers, boards));
}

#[test]
fn part_one_test() {
    let (draw_numbers, boards) = lines_from_file("../input/test-input.txt");
    assert_eq!(Some(4512), part_one(draw_numbers, boards));
}

#[test]
fn part_two_test() {
    let (draw_numbers, boards) = lines_from_file("../input/test-input.txt");
    assert_eq!(Some(1924), part_two(draw_numbers, boards));
}
