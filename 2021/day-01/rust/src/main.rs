#![allow(dead_code)]
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let list: Vec<usize> = BufReader::new(File::open("../input/input.txt").unwrap())
        .lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .collect();

    println!("{:?}", part_one(list.as_slice()));
    println!("{:?}", part_two(list.as_slice()));
}

fn part_one(list: &[usize]) -> usize {
    list.windows(2).fold(
        0,
        |acc, window| {
            if window[0] < window[1] {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn part_two(list: &[usize]) -> usize {
    let mut previous_sum = usize::max_value();
    let mut increase_count = 0;

    for window in list.windows(3) {
        let current_sum = window.iter().sum();
        if current_sum > previous_sum {
            increase_count += 1;
        }
        previous_sum = current_sum;
    }

    increase_count
}

#[test]
fn part_one_test() {
    let list: Vec<usize> = BufReader::new(File::open("../input/test-input.txt").unwrap())
        .lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .collect();

    assert_eq!(part_one(list.as_slice()), 7);
}

#[test]
fn part_two_test() {
    let list: Vec<usize> = BufReader::new(File::open("../input/test-input.txt").unwrap())
        .lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .collect();

    assert_eq!(part_two(list.as_slice()), 5);
}

