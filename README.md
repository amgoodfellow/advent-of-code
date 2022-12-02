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

## Day 1: Calorie Counting

Parts 1 and 2

For the first day or two, the parts probably won't need to be split up

``` rust
let mut elves: Vec<usize> = vec![0];
let mut elf_name = 0;

for line in std::fs::read_to_string("../input/puzzle").unwrap().lines() {
    if let Some(number) = line.parse::<usize>().ok() {
        elves[elf_name] += number;
    } else {
        elf_name += 1;
        elves.push(0)
    }
}

elves.sort_by(|a, b| b.cmp(a));

let max_elf = elves.get(0);
let max_three_elves: usize = elves[0..3].iter().sum();

println!("Part one: {:?}", max_elf);
println!("Part two: {:?}", max_three_elves);
```
