---
title: Advent of Code
---

# Layout

At the root of this project directory, there are separate folders for
each year:

|      |
|------|
| 2020 |
| 2021 |

Inside each year, there are folders for each day following the format
`day-xx`

|         |
|---------|
| cday-01 |
| day-01  |
| day-02  |
| day-03  |
| day-04  |
| day-05  |

Inside of each day will be a folder for the language that that day's
puzzle was completed in, as well as a folder for puzzle inputs. Often
the language will be Rust, but there might be different languages for
each day. I think AoC is fun for seeing how different languages nudge
you in different directions. Each Rust solution will be a full cargo
project, just because I think it's more convenient than breaking things
into workspaces.

# Current Day

## Day 5: Hydrothermal Venture

### Rust

Today I decided to mess around a long time on parsing the file in a
slightly more proper way. Instead of doing all the parsing logic in some
sort of `read_lines` function, I actually implemented the `FromStr`
trait on a `Line` object so that I could use the `parse` method.

``` rust
impl FromStr for Line {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("Invalid regex");
        if let Some(cap) = re.captures(s) {
            let mut x1 = cap[1].parse::<usize>()?;
            let mut y1 = cap[2].parse::<usize>()?;
            let mut x2 = cap[3].parse::<usize>()?;
            let mut y2 = cap[4].parse::<usize>()?;
            if x1 == x2 {
                if y1 > y2 {
                    std::mem::swap(&mut y1, &mut y2);
                }
            }
            if y1 == y2 {
                if x1 > x2 {
                    std::mem::swap(&mut x1, &mut x2);
                }
            }
            return Ok(Line::with(x1, x2, y1, y2));
        }
        Err(ParserError {
            message: "Bad line".to_string(),
        })
    }
}
```

It actually wasn't as bad as I thought. I'll probably try to keep doing
it this way if I end up parsing the input file to some custom struct

Oh, also a note - the little x1/x2 y1/y2 swap was a quick hack so I
wouldn't have to worry about incorrect bounds on ranges for part 1. In
part 2 I ended up having to deal with it anyway. A good idea if I have
extra time would be to move this logic from here into `part_one`.

1.  Part 1

    ``` rust
    fn part_one(lines: &[Line]) -> usize {
        let mut seafloor = vec![vec![0; 1000]; 1000];

        for line in lines {
            if line.x1 == line.x2 {
                for row_index in line.y1..=line.y2 {
                    seafloor[row_index][line.x1] += 1;
                }
            } else if line.y1 == line.y2 {
                for column_index in line.x1..=line.x2 {
                    seafloor[line.y1][column_index] += 1;
                }
            }
        }

        seafloor.iter().fold(0, |acc, element| {
            acc + element.iter().filter(|x| x > &&1).count()
        })
    }
    ```

2.  Part 2

    ``` rust
    fn part_two(lines: &[Line]) -> usize {
        let mut seafloor = vec![vec![0; 1000]; 1000];

        for line in lines {
            if line.x1 == line.x2 {
                for row_index in line.y1..=line.y2 {
                    seafloor[row_index][line.x1] += 1;
                }
            } else if line.y1 == line.y2 {
                for column_index in line.x1..=line.x2 {
                    seafloor[line.y1][column_index] += 1;
                }
            } else {
                let x_positive = line.x1 < line.x2;
                let y_positive = line.y1 < line.y2;

                for step in 0..=(line.x1 as i32 - line.x2 as i32).abs() as usize {
                    seafloor[if y_positive {
                        line.y1 + step
                    } else {
                        line.y1 - step
                    }][if x_positive {
                        line.x1 + step
                    } else {
                        line.x1 - step
                    }] += 1;
                }
            }
        }

        seafloor.iter().fold(0, |acc, element| {
            acc + element.iter().filter(|x| x > &&1).count()
        })
    }
    ```
