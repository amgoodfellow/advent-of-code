#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|c| c.chars().collect())
        .collect()
}

fn is_closing(symbol: char) -> bool {
    symbol == '>' || symbol == ']' || symbol == '}' || symbol == ')'
}

fn are_matching(open: char, close: char) -> bool {
    (open == '<' && close == '>')
        || (open == '[' && close == ']')
        || (open == '(' && close == ')')
        || (open == '{' && close == '}')
}

fn error_value(c: char) -> usize {
    match c {
        '<' | '>' => 25137,
        '{' | '}' => 1197,
        '[' | ']' => 57,
        '(' | ')' => 3,
        _ => panic!("ALIEN CODE DETECTED"),
    }
}

fn missing_value(c: char) -> usize {
    match c {
        '<' | '>' => 4,
        '{' | '}' => 3,
        '[' | ']' => 2,
        '(' | ')' => 1,
        _ => panic!("ALIEN CODE DETECTED"),
    }
}

// just finding balanced parens
fn part_one(input: Vec<Vec<char>>) -> usize {
    let mut bad_symbols = vec![];
    for line in input {
        let mut stack = VecDeque::new();
        for symbol in line.into_iter() {
            if !is_closing(symbol) {
                stack.push_back(symbol);
            } else {
                if let Some(last_char) = stack.pop_back() {
                    if !are_matching(last_char, symbol) {
                        bad_symbols.push(symbol);
                        break;
                    }
                } else {
                    bad_symbols.push(symbol);
                    break;
                }
            }
        }
    }

    bad_symbols
        .into_iter()
        .fold(0, |acc, symbol| acc + error_value(symbol))
}

fn part_two(input: Vec<Vec<char>>) -> usize {
    let mut missing_values = vec![];
    for line in input {
        let mut stack = VecDeque::new();
        for symbol in line.into_iter() {
            if !is_closing(symbol) {
                stack.push_back(symbol);
            } else {
                if let Some(last_char) = stack.pop_back() {
                    if !are_matching(last_char, symbol) {
                        stack.clear();
                        break;
                    }
                } else {
                    stack.clear();
                    break;
                }
            }
        }
        // The remaining elements in the stack are missing closing tags

        if stack.len() > 0 {
            let mut missing_sum = 0;

            while let Some(current) = stack.pop_back() {
                missing_sum = (missing_sum * 5) + missing_value(current);
            }

            missing_values.push(missing_sum);
        }
    }

    missing_values.sort();

    missing_values[missing_values.len() / 2]
}

fn main() {
    println!(
        "Part one: {:?}",
        part_one(lines_from_file("../input/input.txt"))
    );
    println!(
        "Part two: {:?}",
        part_two(lines_from_file("../input/input.txt"))
    );
}

#[test]
fn part_one_test() {
    assert_eq!(26397, part_one(lines_from_file("../input/test-input.txt")));
}

#[test]
fn part_two_test() {
    assert_eq!(288957, part_two(lines_from_file("../input/test-input.txt")));
}
