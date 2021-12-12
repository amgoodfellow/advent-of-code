#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    convert::TryInto,
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Vec<usize>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|c| {
            c.chars()
                .map(|e| e.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn valid_index(row: i32, col: i32) -> bool {
    if row < 0 || row > 9 || col < 0 || col > 9 {
        false
    } else {
        true
    }
}

fn flashy_flash(input: &mut Vec<Vec<usize>>, flash_count: usize) -> (&Vec<Vec<usize>>, usize) {
    let mut new_flashes = 0;
    for row in 0..input.len() {
        for column in 0..input[0].len() {
            if input[row][column] > 9 {
                new_flashes += 1;
                input[row][column] = 0;

                // increment_adjacent
                for i in -1i32..=1 {
                    for j in -1i32..=1 {
                        let current_row = row as i32 + i;
                        let current_column = column as i32 + j;
                        if valid_index(current_row, current_column) {
                            let octo = input[current_row as usize][current_column as usize];
                            if octo != 0 {
                                input[current_row as usize][current_column as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    if new_flashes > 0 {
        flashy_flash(input, flash_count + new_flashes)
    } else {
        (input, flash_count)
    }
}

fn increment_all(input: &mut Vec<Vec<usize>>) {
    for row in 0..input.len() {
        for column in 0..input[0].len() {
            input[row][column] += 1;
        }
    }
}

fn part_one(mut input: Vec<Vec<usize>>) -> usize {
    let mut flashes = 0;
    for _ in 1..=100 {
        increment_all(&mut input);
        flashes += flashy_flash(&mut input, 0).1;
    }
    flashes
}

fn part_two(mut input: Vec<Vec<usize>>) -> usize {
    let mut flashes = 0;
    let mut step = 0;
    let mut chaos = true;
    while chaos {
        step += 1;
        increment_all(&mut input);
        flashes += flashy_flash(&mut input, 0).1;
        chaos = !input
            .iter()
            .map(|l| l.iter().all(|x| x == &0))
            .all(|x| x == true);
    }
    step
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
    assert_eq!(1656, part_one(lines_from_file("../input/test-input.txt")));
}

#[test]
fn part_two_test() {
    assert_eq!(195, part_two(lines_from_file("../input/test-input.txt")));
}
