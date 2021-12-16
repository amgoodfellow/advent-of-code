#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    path::Path,
};

use regex::Regex;
fn lines_from_file(filename: impl AsRef<Path>) -> (HashSet<(usize, usize)>, Vec<Axis>) {
    let coords = std::fs::read_to_string(&filename)
        .unwrap()
        .lines()
        .filter(|line| line.contains(","))
        .map(|cs| {
            let coords = cs.split(",").collect::<Vec<&str>>();
            (
                coords[0].parse::<usize>().unwrap(),
                coords[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<HashSet<(usize, usize)>>();

    let re = Regex::new(r"fold along (\w)=(\d+)").expect("Invalid regex");

    let folds = std::fs::read_to_string(&filename)
        .unwrap()
        .lines()
        .filter(|line| line.contains("fold"))
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let axis = &captures[1];
            let num = captures[2].parse::<usize>().unwrap();
            match axis {
                "y" => Axis::Y(num),
                "x" => Axis::X(num),
                _ => panic!("Invalid thermal camera instructions. Please get a refund"),
            }
        })
        .collect::<Vec<Axis>>();

    (coords, folds)
}

#[derive(Clone, Copy, Debug)]
enum Axis {
    X(usize),
    Y(usize),
}

fn prett_print(set: &HashSet<(usize, usize)>) {
    let max_x = set.iter().fold(0, |acc, (x, _)| acc.max(*x));
    let max_y = set.iter().fold(0, |acc, (_, y)| acc.max(*y));

    for y in 0..=max_y {
        println!("");
        for x in 0..=max_x {
            print!("{}", if set.contains(&(x, y)) { "#" } else { "." });
        }
    }
    println!("")
}

// I think I can keep track of folds without having to actually make the 2d array
fn part_one(coordinates: &HashSet<(usize, usize)>, fold: Axis) -> usize {
    let mut new_coords: HashSet<(usize, usize)> = HashSet::new();
    match fold {
        Axis::X(fold) => {
            for (x, y) in coordinates {
                // never going to be equal
                let x = if x > &fold { fold - (x - fold) } else { *x };
                new_coords.insert((x, *y));
            }
        }
        Axis::Y(fold) => {
            for (x, y) in coordinates {
                // never going to be equal
                let y = if y > &fold { fold - (y - fold) } else { *y };
                new_coords.insert((*x, y));
            }
        }
    };

    new_coords.len()
}

// I think I can keep track of folds without having to actually make the 2d array
fn part_two<'a>(coordinates: &'a mut HashSet<(usize, usize)>, folds: Vec<Axis>) {
    for fold in folds {
        let mut new_coords: HashSet<(usize, usize)> = HashSet::new();
        match fold {
            Axis::X(fold) => {
                for (x, y) in coordinates.iter() {
                    // never going to be equal
                    let x = if x > &fold { fold - (x - fold) } else { *x };
                    new_coords.insert((x, *y));
                }
            }
            Axis::Y(fold) => {
                for (x, y) in coordinates.iter() {
                    // never going to be equal
                    let y = if y > &fold { fold - (y - fold) } else { *y };
                    new_coords.insert((*x, y));
                }
            }
        };
        *coordinates = new_coords.clone();
    }

    prett_print(coordinates);
}

fn main() {
    let (coords, folds) = lines_from_file("../input/input.txt");
    // guessed 733 was too high
    //println!("\n\n\n{}", part_one(&coords, folds[0]));
    let (mut coords, folds) = lines_from_file("../input/input.txt");
    part_two(&mut coords, folds);
}

#[test]
fn part_one_test() {
    let (coords, folds) = lines_from_file("../input/test-input.txt");
    assert_eq!(17, part_one(&coords, folds[0]));
}

#[test]
fn part_two_test() {
    let (coords, folds) = lines_from_file("../input/test-input.txt");
}
