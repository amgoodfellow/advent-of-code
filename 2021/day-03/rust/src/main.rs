#![allow(dead_code)]
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> std::io::Result<Vec<Vec<usize>>> {
    Ok(BufReader::new(File::open(filename)?)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|j| j.to_digit(2).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>())
}

fn main() {
    let lines = lines_from_file("../input/input.txt").expect("Couldn't read from file");
    let lines2 = lines_from_file("../input/input.txt").expect("Couldn't read from file");
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(lines, lines2, 0));
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
                _ => panic!("Unexpected input"),
            }
        }
        gamma.push(if zeroes > ones { '0' } else { '1' });
        epsilon.push(if zeroes > ones { '1' } else { '0' });
    }

    Some(usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap())
}

fn part_two(oxygen: Vec<Vec<usize>>, scrubber: Vec<Vec<usize>>, column: usize) -> usize {
    // recursion base case
    if oxygen.len() == 1 && scrubber.len() == 1 {
        // take both arrays and condense them into strings
        let o2_string = oxygen[0].iter().map(|x| x.to_string()).collect::<String>();
        let scrubber_string = scrubber[0]
            .iter()
            .map(|x| x.to_string())
            .collect::<String>();

        // return the strings as a single usize
        return usize::from_str_radix(&o2_string, 2).unwrap()
            * usize::from_str_radix(&scrubber_string, 2).unwrap();
    }

    let mut oxygen_final = oxygen;

    // If there are more oxygen numbers to filter
    if oxygen_final.len() > 1 {
        let mut oxygen_zeroes = vec![];
        let mut oxygen_ones = vec![];

        for row in oxygen_final {
            if row[column] == 0 {
                oxygen_zeroes.push(row);
            } else {
                oxygen_ones.push(row);
            }
        }

        oxygen_final = if oxygen_zeroes.len() > oxygen_ones.len() {
            oxygen_zeroes
        } else {
            oxygen_ones
        };
    }

    let mut scrubber_final = scrubber;

    // if there are more co2 numbers to filter
    if scrubber_final.len() > 1 {
        let mut scrubber_zeroes = vec![];
        let mut scrubber_ones = vec![];

        for row in scrubber_final {
            if row[column] == 0 {
                scrubber_zeroes.push(row);
            } else {
                scrubber_ones.push(row);
            }
        }

        scrubber_final = if scrubber_zeroes.len() > scrubber_ones.len() {
            scrubber_ones
        } else {
            scrubber_zeroes
        };
    }

    // recurse
    part_two(oxygen_final, scrubber_final, column + 1)
}

#[test]
fn part_one_test() {
    let lines = lines_from_file("../input/test-input.txt").expect("Couldn't read from file");
    assert_eq!(Some(198), part_one(&lines));
}

#[test]
fn part_two_test() {
    let lines = lines_from_file("../input/test-input.txt").expect("Couldn't read from file");
    let lines2 = lines_from_file("../input/test-input.txt").expect("Couldn't read from file");
    assert_eq!((230), part_two(lines, lines2, 0));
}
