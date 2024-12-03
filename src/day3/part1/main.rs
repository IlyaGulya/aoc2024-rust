use regex::Regex;

fn main() {
    let valid_mul_regex = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();
    let result = valid_mul_regex
        .captures_iter(include_str!("../input.txt"))
        .map(|cap| {
            (
                cap["first"].parse::<u32>().unwrap(),
                cap["second"].parse::<u32>().unwrap(),
            )
        })
        .map(|(x, y)| x * y)
        .sum::<u32>();

    println!("{}", result);
}
