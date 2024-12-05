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

    let fixed_invalid = updates
        .iter()
        .filter_map(|update| {
            let mut sorted = update.clone();
            sorted.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    std::cmp::Ordering::Less
                } else if rules.contains(&(*b, *a)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });

            if *update != sorted {
                Some(sorted)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let middles = fixed_invalid
        .iter()
        .map(|update| update[update.len() / 2])
        .collect::<Vec<_>>();

    let sum = middles.iter().sum::<u32>();

    println!("rules: {:?}", rules);
    println!("updates: {:?}", updates);
    println!("fixed_invalid: {:?}", fixed_invalid);
    println!("middles: {:?}", middles);
    println!("sum: {:?}", sum);
}
