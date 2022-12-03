use std::collections::HashSet;

// Could this be a HashMap and avoid the iteration for position? Yes
// But. Constructing a global char slice is easier
const ALPHABET: &'static [char] = &[
    '.', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn get_priority(e: char) -> Option<usize> {
    ALPHABET.iter().position(|a| a == &e)
}

fn part_one() -> usize {
    std::fs::read_to_string("../input/sample")
        .unwrap()
        .lines()
        .filter_map(|line| {
            let sets: Vec<HashSet<char>> = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(line.len() / 2)
                .map(|chunk| HashSet::from_iter(chunk.iter().cloned()))
                .collect();
            sets[0].intersection(&sets[1]).cloned().max()
        })
        .filter_map(get_priority)
        .sum()
}

fn part_two() -> usize {
    std::fs::read_to_string("../input/puzzle")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<HashSet<char>>>()
        .chunks(3)
        .filter_map(|chunk| {
            let mut result = chunk[0].clone();
            result.retain(|c| chunk[1].contains(c) && chunk[2].contains(c));
            result.iter().cloned().max()
        })
        .filter_map(get_priority)
        .sum()
}

fn _old_part_two() -> usize {
    let mut sacks: Vec<HashSet<char>> = std::fs::read_to_string("../input/puzzle")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let length = sacks.len();

    let mut boyos: Vec<char> = vec![];
    for index in (0..length).step_by(3) {
        let mut result = sacks[index].clone();
        result.retain(|c| sacks[index + 1].contains(c) && sacks[index + 2].contains(c));
        let c: char = result.iter().map(|c| c.clone()).collect::<Vec<char>>()[0];
        boyos.push(c);
    }

    let alphabet: Vec<char> = ".abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let total: usize = boyos
        .iter()
        .filter_map(|c| alphabet.iter().position(|a| c == a))
        .sum();

    total
}

fn main() {
    println!("{:?}", part_one());
    println!("{:?}", part_two());
}
