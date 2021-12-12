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

## Day 11: Dumbo Octopus

Both parts rely on this recursive function, which is not optimal and
poorly named :)

``` rust
fn flashy_flash(input: &mut Vec<Vec<usize>>, flash_count: usize) -> (&Vec<Vec<usize>>, usize) {
    let mut new_flashes = 0;
    for row in 0..input.len() {
        for column in 0..input[0].len() {
            if input[row][column] > 9 {
                new_flashes += 1;
                input[row][column] = 0;

                // increment_adjacent
                for i in -1i32..=1 {
                    for j in -1i32..=1 {
                        let current_row = row as i32 + i;
                        let current_column = column as i32 + j;
                        if valid_index(current_row, current_column) {
                            let octo = input[current_row as usize][current_column as usize];
                            if octo != 0 {
                                input[current_row as usize][current_column as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    if new_flashes > 0 {
        flashy_flash(input, flash_count + new_flashes)
    } else {
        (input, flash_count)
    }
}
```

Part 1

``` rust
fn part_one(mut input: Vec<Vec<usize>>) -> usize {
    let mut flashes = 0;
    for _ in 1..=100 {
        increment_all(&mut input);
        flashes += flashy_flash(&mut input, 0).1;
    }
    flashes
}
```

Part 2

``` rust
fn part_two(mut input: Vec<Vec<usize>>) -> usize {
    let mut step = 0;
    let mut chaos = true;
    while chaos {
        step += 1;
        increment_all(&mut input);
        flashes += flashy_flash(&mut input, 0).1;
        chaos = !input
            .iter()
            .map(|l| l.iter().all(|x| x == &0))
            .all(|x| x == true);
    }
    step
}
```
