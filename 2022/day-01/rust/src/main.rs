fn main() {
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
}
