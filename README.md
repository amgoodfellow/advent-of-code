---
title: Advent of Code
---

# Layout

At the root of this project directory, there are separate folders for
each year:

Inside each year, there are folders for each day following the format
`day-xx`

Inside of each day will be a folder for the language that that day's
puzzle was completed in, as well as a folder for puzzle inputs. Often
the language will be Rust, but there might be different languages for
each day. I think AoC is fun for seeing how different languages nudge
you in different directions. Each Rust solution will be a full cargo
project, just because I think it's more convenient than breaking things
into workspaces.

# Latest Solved Problem:

## Day 10: Syntax Scoring

Part 1

``` rust
fn part_one(input: Vec<Vec<char>>) -> usize {
    let mut bad_symbols = vec![];
    for line in input {
        let mut stack = VecDeque::new();
        for symbol in line.into_iter() {
            if !is_closing(symbol) {
                stack.push_back(symbol);
            } else {
                if let Some(last_char) = stack.pop_back() {
                    if !are_matching(last_char, symbol) {
                        bad_symbols.push(symbol);
                        break;
                    }
                } else {
                    bad_symbols.push(symbol);
                    break;
                }
            }
        }
    }

    bad_symbols
        .into_iter()
        .fold(0, |acc, symbol| acc + error_value(symbol))
}
```

Part 2

``` rust
fn part_two(input: Vec<Vec<char>>) -> usize {
    let mut missing_values = vec![];
    for line in input {
        let mut stack = VecDeque::new();
        for symbol in line.into_iter() {
            if !is_closing(symbol) {
                stack.push_back(symbol);
            } else {
                if let Some(last_char) = stack.pop_back() {
                    if !are_matching(last_char, symbol) {
                        stack.clear();
                        break;
                    }
                } else {
                    stack.clear();
                    break;
                }
            }
        }
        // The remaining elements in the stack are missing closing tags

        if stack.len() > 0 {
            let mut missing_sum = 0;

            while let Some(current) = stack.pop_back() {
                missing_sum = (missing_sum * 5) + missing_value(current);
            }

            missing_values.push(missing_sum);
        }
    }

    missing_values.sort();

    missing_values[missing_values.len() / 2]
}
```
