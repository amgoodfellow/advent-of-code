#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    path::Path,
};

use regex::Regex;
fn lines_from_file(filename: impl AsRef<Path>) -> HashMap<String, Vec<String>> {
    let regular_expression = Regex::new(r"(.*)-(.*)").expect("Invalid regex");
    let mut graph = HashMap::new();
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .for_each(|mapping| {
            let captures = regular_expression.captures(mapping).unwrap();
            graph
                .entry(captures[1].to_string())
                .or_insert(vec![])
                .push(captures[2].to_string());
            graph
                .entry(captures[2].to_string())
                .or_insert(vec![])
                .push(captures[1].to_string());
        });

    graph
}

fn part_one(
    map: &HashMap<String, Vec<String>>,
    current: &str,
    goal: &str,
    paths: &mut Vec<String>,
    visited: &mut HashSet<String>,
    total: &mut usize,
) -> usize {
    visited.insert(current.to_string());
    paths.push(current.to_string());

    if current == goal {
        *total += 1;
    } else {
        for node in map.get(current).unwrap() {
            if !visited.contains(node) || !node.chars().all(char::is_lowercase) {
                part_one(map, node, goal, paths, visited, total);
            }
        }
    }

    visited.remove(current);
    paths.pop();

    *total
}

fn part_two(
    map: &HashMap<String, Vec<String>>,
    current: &str,
    goal: &str,
    paths: &mut Vec<String>,
    visited: &mut HashMap<String, usize>,
    total: &mut usize,
) -> usize {
    *visited.entry(current.to_string()).or_insert(0) += 1;
    paths.push(current.to_string());

    let has_circled_back = visited
        .iter()
        .any(|(s, v)| s.chars().all(char::is_lowercase) && v > &1);

    if current == goal {
        *total += 1;
    } else {
        for node in map.get(current).unwrap() {
            if node != "start" {
                let was_visited = visited.contains_key(node) && visited.get(node).unwrap() >= &1;
                let is_small = node.chars().all(char::is_lowercase);
                if !is_small
                    || (is_small && !was_visited)
                    || (is_small && was_visited && !has_circled_back)
                {
                    part_two(map, node, goal, paths, visited, total);
                }
            }
        }
    }

    *visited.entry(current.to_string()).or_insert(1) -= 1;

    paths.pop();

    *total
}

fn main() {
    let mut paths: Vec<String> = vec![];
    let mut visited: HashSet<String> = HashSet::new();
    println!(
        "{:?}",
        part_one(
            &lines_from_file("../input/input.txt"),
            "start",
            "end",
            &mut paths,
            &mut visited,
            &mut 0,
        )
    );
    let mut paths: Vec<String> = vec![];
    let mut visited: HashMap<String, usize> = HashMap::new();
    println!(
        "{:?}",
        part_two(
            &lines_from_file("../input/input.txt"),
            "start",
            "end",
            &mut paths,
            &mut visited,
            &mut 0,
        )
    );
}

#[test]
fn part_one_test() {
    let mut paths: Vec<String> = vec![];
    let mut visited: HashSet<String> = HashSet::new();
    assert_eq!(
        226,
        part_one(
            &lines_from_file("../input/test-input.txt"),
            "start",
            "end",
            &mut paths,
            &mut visited,
            &mut 0
        )
    );
}

#[test]
fn part_two_test() {
    let mut paths: Vec<String> = vec![];
    let mut visited: HashMap<String, usize> = HashMap::new();
    assert_eq!(
        36,
        part_two(
            &lines_from_file("../input/test-input.txt"),
            "start",
            "end",
            &mut paths,
            &mut visited,
            &mut 0
        )
    );
}
