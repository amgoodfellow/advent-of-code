#![allow(dead_code)]
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<usize> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|c| c.trim().parse::<usize>().unwrap())
        .collect()
}

fn part_one(positions: &[usize]) -> usize {
    // Get min / max in crab positions
    let mut min = usize::max_value();
    let mut max = usize::min_value();
    for position in positions {
        if position > &max {
            max = *position;
        }
        if position < &min {
            min = *position;
        }
    }

    // determine total fuel expended for each possibility between
    // min and max
    let difference = max - min + 1;
    let mut position_options = vec![0; difference];

    for index in 0..difference {
        let current_position = min + index;
        position_options[index] = positions
            .iter()
            .map(|x| (current_position as i32 - *x as i32).abs() as usize)
            .sum::<usize>();
    }

    *position_options.iter().min().expect("No minimum found")
}

fn part_two(positions: &[usize]) -> usize {
    // Get min / max in crab positions
    let mut min = usize::max_value();
    let mut max = usize::min_value();
    for position in positions {
        if position > &max {
            max = *position;
        }
        if position < &min {
            min = *position;
        }
    }

    // determine total fuel expended for each possibility between
    // min and max
    let difference = max - min + 1;
    let mut position_options = vec![0; difference];

    for index in 0..difference {
        let current_position = min + index;
        position_options[index] = positions
            .iter()
            .map(|x: &usize| {
                (0..=(current_position as i32 - *x as i32).abs() as usize).sum::<usize>()
            })
            .sum::<usize>();
    }

    *position_options.iter().min().expect("No minimum found")
}

fn main() {
    println!(
        "Part one: {:?}",
        part_one(lines_from_file("../input/input.txt").as_slice())
    );
    println!(
        "Part two: {:?}",
        part_two(lines_from_file("../input/input.txt").as_slice())
    );
}

#[test]
fn part_one_test() {
    assert_eq!(
        37,
        part_one(lines_from_file("../input/test-input.txt").as_slice())
    );
}

#[test]
fn part_two_test() {
    assert_eq!(
        168,
        part_one(lines_from_file("../input/test-input.txt").as_slice())
    );
}
