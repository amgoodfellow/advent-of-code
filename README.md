---
title: Advent of Code
---

# Layout

At the root of this project directory, there are separate folders for
each year.

Inside each year, there are folders for each day following the format
`day-xx`

Inside of each day will be a folder for the language the day's puzzle
was completed in, as well as a folder for puzzle inputs. Often the
language will be Rust, but there might be extra ones if I was feeling
very motivated. I think AoC is fun for seeing how different languages
nudge you in different directions. Each Rust solution will be a full
cargo project, just because I think it's more convenient than breaking
things into workspaces.

Also! A new change starting in `2022/` will be the addition of a [Nix
Flake](https://nixos.wiki/wiki/Flakes)

# Latest Solved Problem:

## Day 3: Rucksack Reorganization

### Setup

Since prioritization is based on the index of a character in an
alphabet, it seemed like we'd need a good way to get that index. Note
the `.` at the front of the alphabet so I wouldn't have to add one to
indices lol

``` rust
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
```

### Part 1

My initial solution was a bit messier. I first created a
`Vec<Vec<char>>` from the input file, and then iterated over that
creating the hashmaps and looking at the results of the intersection.

This cleaned up solution makes the whole thing one expression, which is
kinda fun

``` rust
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
```

### Part 2

The simple solution here would have been to stick with set intersection.
For sets `a, b, c`, you'd simply perform something like:

``` rust
let result = a.intersection(b).intersection(c).max()
```

However, there were some issues with chaining intersections in this way.
Since it was late, I decided to use the `HashSet::retain()` function
which mutates `self`, and removes everything not matching the predicate.

It actually turned out to be a pretty tidy solution, I think.

``` rust
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
```
