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

# Current Day

## Day 9: Smoke Basin

Part 1

Per-usual, my initial thought is to do this the most straightforward way
I can.

Iterate through the 2d array and for each element check to see if all
adjacent elements are bigger

``` rust
fn part_one(map: Vec<Vec<usize>>) -> usize {
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
```

Part 2

``` rust
```
