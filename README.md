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

Oh boi this is a pretty rough solution, but I'm running behind on these
challenges, so it'll do for now

``` rust
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
```
