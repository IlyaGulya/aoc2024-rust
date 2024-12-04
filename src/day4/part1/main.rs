fn main() {
    let matrix = include_str!("../input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();

    let max_i = (&matrix[0].len() - 1) as isize;
    let max_j = (&matrix.len() - 1) as isize;

    let mut count = 0u32;

    for ui in 0..matrix.len() {
        for uj in 0..matrix[ui].len() {
            if matrix[ui][uj] != b'X' {
                continue;
            }

            let i = ui as isize;
            let j = uj as isize;

            let lookup = [
                // Up
                [(i - 1, j), (i - 2, j), (i - 3, j)],
                // Right
                [(i, j + 1), (i, j + 2), (i, j + 3)],
                // Down
                [(i + 1, j), (i + 2, j), (i + 3, j)],
                // Left
                [(i, j - 1), (i, j - 2), (i, j - 3)],
                // Up-Right
                [(i - 1, j + 1), (i - 2, j + 2), (i - 3, j + 3)],
                // Down-Right
                [(i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)],
                // Down-Left
                [(i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)],
                // Up-Left
                [(i - 1, j - 1), (i - 2, j - 2), (i - 3, j - 3)],
            ];

            for indices in lookup.iter() {
                let word = indices
                    .iter()
                    .filter(|(i, j)| i <= &max_i && j <= &max_j && i >= &0 && j >= &0)
                    .map(|(i, j)| matrix[*i as usize][*j as usize])
                    .collect::<Vec<u8>>();

                if word == [b'M', b'A', b'S'] {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
