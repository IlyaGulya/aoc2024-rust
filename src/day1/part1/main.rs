use std::iter::zip;

fn main() {
    let pairs =
        include_str!("input.txt")
            .lines()
            .map(|line| line.split_once("   ").unwrap())
            .map(|(s, v)| (s.parse::<u32>().unwrap(), v.parse::<u32>().unwrap()));

    let mut first_seq =
        pairs
            .clone()
            .map(|(a, _)| a)
            .collect::<Vec<u32>>();

    first_seq.sort();

    let mut second_seq =
        pairs
            .clone()
            .map(|(_, b)| b)
            .collect::<Vec<u32>>();

    second_seq.sort();

    let dist: u32 =
        zip(first_seq, second_seq)
            .map(|(a, b)| a.abs_diff(b))
            .sum();

    println!("{}", dist);
}
