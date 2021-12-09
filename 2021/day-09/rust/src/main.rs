#![allow(dead_code)]
use regex::{Match, Regex};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    str::FromStr,
};

fn part_one<A>(filename: A) -> usize
where
    A: AsRef<Path>,
{
    let map = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            println!("{:?}", line);
            line.split_terminator("")
                .filter_map(|digit| digit.parse::<usize>().ok())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut low_points = vec![];

    for row in 0..map.len() {
        for column in 0..map[0].len() {
            let current = map[row][column];
            let left = if column > 0 {
                map[row].get(column - 1).unwrap_or(&usize::max_value())
            } else {
                &usize::max_value()
            };
            let right = map[row].get(column + 1).unwrap_or(&usize::max_value());
            let above = if row > 0 {
                map.get(row - 1)
                    .and_then(|v| v.get(column))
                    .unwrap_or(&usize::max_value())
            } else {
                &usize::max_value()
            };
            let below = map
                .get(row + 1)
                .and_then(|v| v.get(column))
                .unwrap_or(&usize::max_value());

            if current < *left && current < *right && current < *above && current < *below {
                low_points.push(current.clone() + 1);
            }
        }
    }

    low_points.iter().sum()
}

fn main() {
    println!("Part one: {:?}", part_one("../input/input.txt"));
}

#[test]
fn part_one_test() {
    assert_eq!(15, part_one("../input/test-input.txt"));
}

//#[test]
//fn part_two_test() {
//    assert_eq!(
//        168,
//        part_one("../input/test-input.txt")
//    );
//}
