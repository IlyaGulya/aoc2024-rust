use std::collections::HashMap;

fn main() {
    let pairs =
        include_str!("../input.txt")
            .lines()
            .map(|line| line.split_once("   ").unwrap())
            .map(|(s, v)| (s.parse::<u32>().unwrap(), v.parse::<u32>().unwrap()));

    let first_seq =
        pairs
            .clone()
            .map(|(a, _)| a)
            .collect::<Vec<u32>>();

    let second_counts =
        pairs
            .clone()
            .map(|(_, b)| b)
            .fold(HashMap::new(), |mut acc, b| {
                *acc.entry(b).or_insert(0) += 1;
                acc
            });

    let score =
        first_seq
            .iter()
            .map(|a| a * second_counts.get(a).unwrap_or(&0))
            .sum::<u32>();

    println!("{}", score);
}
