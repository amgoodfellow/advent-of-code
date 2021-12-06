#![allow(dead_code)]
use std::fmt::Debug;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use regex::Regex;

// This is cool, but seems like too much work for this
fn lines_from_file_with<T>(
    filename: impl AsRef<Path>,
    closure: &dyn Fn(Result<String, std::io::Error>) -> T,
) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    BufReader::new(File::open(filename).expect("File not found"))
        .lines()
        .map(closure)
        .collect()
}

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

impl From<std::num::ParseIntError> for ParserError {
    fn from(_: std::num::ParseIntError) -> ParserError {
        ParserError {
            message: "Invalid data type".to_string(),
        }
    }
}

#[derive(Debug)]
struct Line {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

impl Line {
    pub fn with(x1: usize, x2: usize, y1: usize, y2: usize) -> Self {
        Line { x1, x2, y1, y2 }
    }
}

impl FromStr for Line {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("Invalid regex");
        if let Some(cap) = re.captures(s) {
            let mut x1 = cap[1].parse::<usize>()?;
            let mut y1 = cap[2].parse::<usize>()?;
            let mut x2 = cap[3].parse::<usize>()?;
            let mut y2 = cap[4].parse::<usize>()?;
            if x1 == x2 {
                if y1 > y2 {
                    std::mem::swap(&mut y1, &mut y2);
                }
            }
            if y1 == y2 {
                if x1 > x2 {
                    std::mem::swap(&mut x1, &mut x2);
                }
            }
            return Ok(Line::with(x1, x2, y1, y2));
        } else {
            Err(ParserError {
                message: "Bad line".to_string(),
            })
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Line> {
    BufReader::new(File::open(filename).expect("File not found"))
        .lines()
        .map(|x| x.unwrap().parse::<Line>().unwrap())
        .collect()
}

fn part_one(lines: &[Line]) -> usize {
    let mut seafloor = vec![vec![0; 1000]; 1000];

    for line in lines {
        if line.x1 == line.x2 {
            for row_index in line.y1..=line.y2 {
                seafloor[row_index][line.x1] += 1;
            }
        } else if line.y1 == line.y2 {
            for column_index in line.x1..=line.x2 {
                seafloor[line.y1][column_index] += 1;
            }
        }
    }

    seafloor.iter().fold(0, |acc, element| {
        acc + element.iter().filter(|x| x > &&1).count()
    })
}

fn part_two(lines: &[Line]) -> usize {
    let mut seafloor = vec![vec![0; 1000]; 1000];

    for line in lines {
        if line.x1 == line.x2 {
            for row_index in line.y1..=line.y2 {
                seafloor[row_index][line.x1] += 1;
            }
        } else if line.y1 == line.y2 {
            for column_index in line.x1..=line.x2 {
                seafloor[line.y1][column_index] += 1;
            }
        } else {
            let x_positive = line.x1 < line.x2;
            let y_positive = line.y1 < line.y2;

            for step in 0..=(line.x1 as i32 - line.x2 as i32).abs() as usize {
                seafloor[if y_positive {
                    line.y1 + step
                } else {
                    line.y1 - step
                }][if x_positive {
                    line.x1 + step
                } else {
                    line.x1 - step
                }] += 1;
            }
        }
    }

    seafloor.iter().fold(0, |acc, element| {
        acc + element.iter().filter(|x| x > &&1).count()
    })
}

fn main() {
    println!("{:?}", part_one(lines_from_file("input.txt").as_slice()));
    println!("{:?}", part_two(lines_from_file("input.txt").as_slice()));
    // got 21384 but was too high
}

#[test]
fn part_one_test() {
    assert_eq!(5, part_one(lines_from_file("test-input.txt").as_slice()));
}

#[test]
fn part_two_test() {
    assert_eq!(12, part_two(lines_from_file("test-input.txt").as_slice()));
}
