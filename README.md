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

## Day 13: Transparent Origami

Part 1

``` rust
//I think I can keep track of folds without having to actually make the 2d array
fn part_one(coordinates: &HashSet<(usize, usize)>, fold: Axis) -> usize {
    let mut new_coords: HashSet<(usize, usize)> = HashSet::new();
    match fold {
        Axis::X(fold) => {
            for (x, y) in coordinates {
                // never going to be equal
                let x = if x > &fold { fold - (x - fold) } else { *x };
                new_coords.insert((x, *y));
            }
        }
        Axis::Y(fold) => {
            for (x, y) in coordinates {
                // never going to be equal
                let y = if y > &fold { fold - (y - fold) } else { *y };
                new_coords.insert((*x, y));
            }
        }
    };

    new_coords.len()
}
```

Part 2

Part two needed a new `pretty_print` function, but other than that was
pretty much the same as part<sub>one</sub>

1.  Pretty Print

    ``` rust
    fn prett_print(set: &HashSet<(usize, usize)>) {
        let max_x = set.iter().fold(0, |acc, (x, _)| acc.max(*x));
        let max_y = set.iter().fold(0, |acc, (_, y)| acc.max(*y));

        for y in 0..=max_y {
            println!("");
            for x in 0..=max_x {
                print!("{}", if set.contains(&(x, y)) { "#" } else { "." });
            }
        }
        println!("")
    }
    ```

2.  Part Two

    ``` rust
    fn part_two<'a>(coordinates: &'a mut HashSet<(usize, usize)>, folds: Vec<Axis>) {
        for fold in folds {
            let mut new_coords: HashSet<(usize, usize)> = HashSet::new();
            match fold {
                Axis::X(fold) => {
                    for (x, y) in coordinates.iter() {
                        // never going to be equal
                        let x = if x > &fold { fold - (x - fold) } else { *x };
                        new_coords.insert((x, *y));
                    }
                }
                Axis::Y(fold) => {
                    for (x, y) in coordinates.iter() {
                        // never going to be equal
                        let y = if y > &fold { fold - (y - fold) } else { *y };
                        new_coords.insert((*x, y));
                    }
                }
            };
            *coordinates = new_coords.clone();
        }

        prett_print(coordinates);
    }
    ```
