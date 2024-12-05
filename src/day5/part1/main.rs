use std::collections::HashSet;

fn main() {
    let mut lines = include_str!("../input.txt").lines().into_iter();

    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect::<HashSet<_>>();

    let updates = lines
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let valid = updates
        .iter()
        .filter(|update| {
            update.windows(2).all(|chunk| {
                let valid = (chunk[0], chunk[1]);
                let invalid = (chunk[1], chunk[0]);

                rules.contains(&valid) || !rules.contains(&invalid)
            })
        })
        .collect::<Vec<_>>();

    let middles = valid
        .iter()
        .map(|update| update[update.len() / 2])
        .collect::<Vec<_>>();

    let sum = middles.iter().sum::<u32>();

    println!("rules: {:?}", rules);
    println!("updates: {:?}", updates);
    println!("valid: {:?}", valid);
    println!("middles: {:?}", middles);
    println!("sum: {:?}", sum);
}
