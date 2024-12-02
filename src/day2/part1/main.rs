#![feature(array_chunks)]
#![feature(iter_map_windows)]

fn main() {
    let safe_count =
        include_str!("../input.txt")
            .lines()
            .map(|line| {
                line
                    .split(' ')
                    .map(|x| x.parse::<i8>().unwrap())
                    .collect::<Vec<i8>>()
            })
            .filter(|x| {
                let expected = x.len() - 2; // we have 2 windows
                let actual =
                    x
                        .windows(2)
                        .map(|w| w[0] - w[1])
                        .take_while(|diff| (1..=3).contains(&diff.abs()))
                        .map(|diff| diff.signum())
                        .map_windows(|[prev, curr]| prev == curr)
                        .take_while(|a| *a)
                        .count();
                return expected == actual;
            })
            .count();

    println!("{:#?}", safe_count);
}