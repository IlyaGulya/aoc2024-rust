use regex::Regex;

#[derive(Debug)]
enum Op {
    Mul(u32, u32),
    Do,
    Dont,
}

fn main() {
    let valid_mul_regex =
        Regex::new(r"((?<do>do)|(?<dont>don't))\(\)|(?<mul>mul\((?<a>\d{1,3}),(?<b>\d{1,3})\))")
            .unwrap();

    let (_, sum) = valid_mul_regex
        .captures_iter(include_str!("../input.txt"))
        .filter_map(|cap| {
            let do_op = cap.name("do").map(|_| Op::Do);
            let dont_op = cap.name("dont").map(|_| Op::Dont);
            let mul_op = cap
                .name("mul")
                .map(|m| Op::Mul(cap["a"].parse().unwrap(), cap["b"].parse().unwrap()));

            do_op.or(dont_op).or(mul_op)
        })
        .fold((true, 0), |(enabled, sum), op| match op {
            Op::Mul(a, b) => {
                if enabled {
                    (enabled, sum + a * b)
                } else {
                    (enabled, sum)
                }
            }
            Op::Do => (true, sum),
            Op::Dont => (false, sum),
        });

    println!("{:?}", sum);
}
