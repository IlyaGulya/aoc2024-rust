use std::iter::zip;

fn main() {
    let (mut first_seq, mut second_seq): (Vec<_>, Vec<_>) =
        include_str!("../input.txt")
            .lines()
            .map(|line| line.split_once("   ").unwrap())
            .map(|(s, v)| (s.parse::<u32>().unwrap(), v.parse::<u32>().unwrap()))
            .unzip();
    
    first_seq.sort();
    second_seq.sort();

    let dist: u32 =
        zip(first_seq, second_seq)
            .map(|(a, b)| a.abs_diff(b))
            .sum();

    println!("{}", dist);
}