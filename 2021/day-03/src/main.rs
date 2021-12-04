#![allow(dead_code)]
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> std::io::Result<Vec<Vec<usize>>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .map(|x| x.unwrap().chars().map(|j| j.to_digit(2).unwrap() as usize).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>())
}

fn main() {
    let lines = lines_from_file("input.txt").expect("Couldn't read from file");
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

fn part_one(lines: &Vec<Vec<usize>>) -> Option<usize> {

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for column in 0..lines[0].len() {
        let mut ones = 0;
        let mut zeroes = 0;

        for row in 0..lines.len() {
            match lines[row][column] {
                0 => zeroes += 1,
                1 => ones += 1,
                _ => panic!("Unexpected input")
            }
        }
        gamma.push( if zeroes > ones { '0' } else { '1' });
        epsilon.push( if zeroes > ones { '1' } else { '0' });
    }

    Some(usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap())
}

fn part_two(lines: &Vec<Vec<usize>>) -> Option<usize> {
    Some(19)
}

#[test]
fn part_one_test() {
    let lines = lines_from_file("test-input.txt").expect("Couldn't read from file");
    assert_eq!(Some(198), part_one(&lines));
}

#[test]
fn part_two_test() {
    let lines = lines_from_file("test-input.txt").expect("Couldn't read from file");
    assert_eq!(Some(900), part_two(lines));
}
