#![allow(dead_code)]
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> std::io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines = lines_from_file("../input/input.txt").expect("Couldn't read from file");
    println!("{:?}", part_one(lines.as_slice()));
    println!("{:?}", part_two(lines.as_slice()));
}

fn part_one(lines: &[String]) -> Option<usize> {
    let re = Regex::new(r"(\w+) (\d+)").expect("Invalid regex");

    let mut vertical = 0;
    let mut depth = 0;

    for line in lines {
        let capture = re.captures(&line)?;
        let direction = &capture[1];
        let amount = capture[2].parse::<usize>().unwrap();

        match direction {
            "forward" => vertical += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("Unexpected direction! Submarine sunk :("),
        }
    }

    Some(vertical * depth)
}

fn part_two(lines: &[String]) -> Option<usize> {
    let re = Regex::new(r"(\w+) (\d+)").expect("Invalid regex");

    let mut vertical = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let capture = re.captures(&line)?;
        let direction = &capture[1];
        let amount = capture[2].parse::<usize>().unwrap();

        match direction {
            "forward" => {
                vertical += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("Unexpected direction! Submarine sunk :("),
        }
    }

    Some(vertical * depth)
}

#[test]
fn part_one_test() {
    let lines = lines_from_file("../input/test-input.txt").expect("Couldn't read from file");
    assert_eq!(Some(150), part_one(lines.as_slice()));
}

#[test]
fn part_two_test() {
    let lines = lines_from_file("../input/test-input.txt").expect("Couldn't read from file");
    assert_eq!(Some(900), part_two(lines.as_slice()));
}
