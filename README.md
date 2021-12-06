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

## Day 6: Lanternfish

### Rust

1.  Part 1 (and 2)

    ``` rust
    fn let_there_be_fish(the_first_ones: &[usize], n: usize) -> usize {
        let mut fishes = vec![0; 9];

        for fish in the_first_ones {
            match fish {
                1 => fishes[1] += 1,
                2 => fishes[2] += 1,
                3 => fishes[3] += 1,
                4 => fishes[4] += 1,
                5 => fishes[5] += 1,
                6 => fishes[6] += 1,
                7 => fishes[7] += 1,
                8 => fishes[8] += 1,
                _ => panic!("AH! That isn't a fish!"),
            }
        }

        for _ in 1..=n {
            let mut new_fishes = vec![0; 9];
            new_fishes[7] = fishes[8];
            new_fishes[6] = fishes[7] + fishes[0];
            new_fishes[5] = fishes[6];
            new_fishes[4] = fishes[5];
            new_fishes[3] = fishes[4];
            new_fishes[2] = fishes[3];
            new_fishes[1] = fishes[2];
            new_fishes[0] = fishes[1];
            new_fishes[8] = fishes[0];
            fishes = new_fishes;
        }

        fishes.iter().sum()
    }
    ```
