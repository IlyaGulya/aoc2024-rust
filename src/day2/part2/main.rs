#![feature(array_chunks)]
#![feature(iter_map_windows)]

fn diffs_valid(diffs: &Vec<i8>) -> bool {
    let first = diffs[0].signum();
    diffs
        .iter()
        .all(|diff| diff.signum() == first && (1..=3).contains(&diff.abs()))
}

fn sequence_valid(seq: &Vec<i8>) -> bool {
    let diffs =
        seq
            .windows(2)
            .map(|w| w[0] - w[1])
            .collect::<Vec<i8>>();

    diffs_valid(&diffs)
}

fn main() {
    let safe_count =
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i8>().unwrap())
                    .collect::<Vec<i8>>()
            })
            .filter_map(|x| {
                if sequence_valid(&x) {
                    return Some((x.clone(), x));
                }

                (0..x.len()).find_map(|i| {
                    let mut y = x.clone();
                    y.remove(i);
                    if sequence_valid(&y) {
                        Some((x.clone(), y))
                    } else {
                        None
                    }
                })
            })
            .count();

    println!("{:#?}", safe_count);
}