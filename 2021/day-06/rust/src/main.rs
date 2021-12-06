#![allow(dead_code)]
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<usize> {
    std::fs::read_to_string(filename)
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn let_there_be_fish(the_first_ones: &[usize], n: usize) -> usize {
    let mut fishes = vec![0; 9];

    for fish in the_first_ones {
        match fish {
            1 => fishes[1] += 1,
            2 => fishes[2] += 1,
            3 => fishes[3] += 1,
            4 => fishes[4] += 1,
            5 => fishes[5] += 1,
            6 => fishes[6] += 1,
            7 => fishes[7] += 1,
            8 => fishes[8] += 1,
            _ => panic!("AH! That isn't a fish!"),
        }
    }

    for _ in 1..=n {
        let mut new_fishes = vec![0; 9];
        new_fishes[7] = fishes[8];
        new_fishes[6] = fishes[7] + fishes[0];
        new_fishes[5] = fishes[6];
        new_fishes[4] = fishes[5];
        new_fishes[3] = fishes[4];
        new_fishes[2] = fishes[3];
        new_fishes[1] = fishes[2];
        new_fishes[0] = fishes[1];
        new_fishes[8] = fishes[0];
        fishes = new_fishes;
    }

    fishes.iter().sum()
}

fn main() {
    println!(
        "Part one: {:?}",
        let_there_be_fish(lines_from_file("../input/input.txt").as_slice(), 80)
    );
    println!(
        "Part two: {:?}",
        let_there_be_fish(lines_from_file("../input/input.txt").as_slice(), 256)
    );
}

#[test]
fn part_one_test() {
    assert_eq!(
        5934,
        let_there_be_fish(lines_from_file("../input/test-input.txt").as_slice(), 80)
    );
}

#[test]
fn part_two_test() {
    assert_eq!(
        26984457539,
        let_there_be_fish(lines_from_file("../input/test-input.txt").as_slice(), 256)
    );
}
