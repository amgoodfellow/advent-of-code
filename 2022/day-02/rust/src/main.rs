fn calculate_score(play: (&str, &str)) -> usize {
    match play {
        ("A", "X") => 1 + 3,
        ("A", "Y") => 2 + 6,
        ("A", "Z") => 3 + 0,
        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,
        ("C", "X") => 1 + 6,
        ("C", "Y") => 2 + 0,
        ("C", "Z") => 3 + 3,
        _ => panic!(),
    }
}

fn calculate_score_two(play: (&str, &str)) -> usize {
    match play {
        ("A", "X") => 3 + 0,
        ("A", "Y") => 1 + 3,
        ("A", "Z") => 2 + 6,
        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,
        ("C", "X") => 2 + 0,
        ("C", "Y") => 3 + 3,
        ("C", "Z") => 1 + 6,
        _ => panic!(),
    }
}

fn part_one() -> Option<(usize, usize)> {
    std::fs::read_to_string("../input/puzzle")
        .unwrap()
        .lines()
        .map(|c| {
            let a: Vec<&str> = c.split_ascii_whitespace().collect();
            (
                calculate_score((a[0], a[1])),
                calculate_score_two((a[0], a[1])),
            )
        })
        .reduce(|(x, y), (a, b)| (x + a, y + b))
}

fn main() {
    println!("{:?}", part_one());
}
