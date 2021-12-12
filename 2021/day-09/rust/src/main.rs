#![allow(dead_code)]
use regex::{Match, Regex};
use std::{
    collections::{HashMap, HashSet, VecDeque},
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

// We can find the coordinates of all low points already
//
// And a basin is an extension from any low point until you reach 9 or an edge
//
// We are given that basins do not overlap
//
// So the plan:
//  for each low point, do a search, returning the count of elements inside
//  return the mulitplication of the three largest basins

fn bfs(map: &Vec<Vec<usize>>, low_point: (usize, usize)) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(low_point);

    let mut visited = HashSet::new();

    while !to_visit.is_empty() {
        let (row, column) = to_visit.pop_front().expect("queue was empty");
        visited.insert((row, column));

        // look at adjacent and push each to queue
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

        if left < &9 && !visited.contains(&(row, column - 1)) {
            to_visit.push_back((row, column - 1));
        }
        if above < &9 && !visited.contains(&(row - 1, column)) {
            to_visit.push_back((row - 1, column));
        }
        if right < &9 && !visited.contains(&(row, column + 1)) {
            to_visit.push_back((row, column + 1));
        }
        if below < &9 && !visited.contains(&(row + 1, column)) {
            to_visit.push_back((row + 1, column));
        }
    }

    visited.len()
}

fn part_two<A>(filename: A) -> usize
where
    A: AsRef<Path>,
{
    let map = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
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
                low_points.push((row, column));
            }
        }
    }

    let mut basin_sizes = low_points
        .into_iter()
        .map(|point| bfs(&map, point))
        .collect::<Vec<usize>>();

    basin_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut product = 1;
    for size in basin_sizes.iter().take(3) {
        product *= size;
    }
    product
}

fn main() {
    println!("Part one: {:?}", part_one("../input/input.txt"));
    println!("Part two: {:?}", part_two("../input/input.txt"));
}

#[test]
fn part_one_test() {
    assert_eq!(15, part_one("../input/test-input.txt"));
}

#[test]
fn part_two_test() {
    assert_eq!(1134, part_two("../input/test-input.txt"));
}
