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

## Day 7: The Treachery of Whales

### Thoughts

So this is an optimization problem to get all the input to the same
number, while minimizing the total number movement

Initial brute-force idea is to consider every position between the
min/max numbers in the array, and see which choice results in the min
number

### Rust

Rust is a perfect language for this day's challenge. \[r(cr)\]ustacians
have to stick together, yanno?

1.  Part 1

    ``` rust
    fn part_one(positions: &[usize]) -> usize {
        // Get min / max in crab positions
        let mut min = usize::max_value();
        let mut max = usize::min_value();
        for position in positions {
            if position > &max {
                max = *position;
            }
            if position < &min {
                min = *position;
            }
        }

        // determine total fuel expended for each possibility between
        // min and max
        let difference = max - min + 1;
        let mut position_options = vec![0; difference];

        for index in 0..difference {
            let current_position = min + index;
            position_options[index] = positions
                .iter()
                .map(|x| (current_position as i32 - *x as i32).abs() as usize)
                .sum::<usize>();
        }

        *position_options.iter().min().expect("No minimum found")
    }
    ```

    So this solution feels pretty brute-force, but it completes too
    quickly for `time` to keep track of so I guess I'm okay with it.

    Also, a weird, thing. I tried to do a `positions.fold()` instead of
    the `map().sum()`, but I was having some weird type error. I was
    probably doing something dumb

2.  Part 2

    > As it turns out, crab submarine engines don't burn fuel at a
    > constant rate. Instead, each change of 1 step in horizontal
    > position costs 1 more unit of fuel than the last

    For this, my initial (lazy) thought is that I can keep everything
    the same and just take my current

    ``` rust
    .map(|x| (current_position as i32 - *x as i32).abs() as usize)
    ```

    and sum the result from 0 to itself

    ``` rust
    .map(|x: &usize| {
        (0..=(current_position as i32 - *x as i32).abs() as usize).sum::<usize>()
    })
    ```

    We'll see if it works

    ``` rust
    fn part_two(positions: &[usize]) -> usize {
        // Get min / max in crab positions
        let mut min = usize::max_value();
        let mut max = usize::min_value();
        for position in positions {
            if position > &max {
                max = *position;
            }
            if position < &min {
                min = *position;
            }
        }

        // determine total fuel expended for each possibility between
        // min and max
        let difference = max - min + 1;
        let mut position_options = vec![0; difference];

        for index in 0..difference {
            let current_position = min + index;
            position_options[index] = positions
                .iter()
                .map(|x: &usize| {
                    (0..=(current_position as i32 - *x as i32).abs() as usize).sum::<usize>()
                })
                .sum::<usize>();
        }

        *position_options.iter().min().expect("No minimum found")
    }
    ```

    So, it *works*. But not very well. It took several seconds to get
    the answer for the full puzzle input. I'll try and come up with a
    better solution going forward
