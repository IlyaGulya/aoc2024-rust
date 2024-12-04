fn main() {
    let matrix: Vec<&[u8]> = include_str!("../input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    const VALID_DIAGONALS: [(u8, u8); 2] = [(b'M', b'S'), (b'S', b'M')];

    let mut count = 0;

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] != b'A' {
                continue;
            }

            let diag1 = (matrix[i - 1][j - 1], matrix[i + 1][j + 1]);
            let diag2 = (matrix[i - 1][j + 1], matrix[i + 1][j - 1]);

            if VALID_DIAGONALS.contains(&diag1) && VALID_DIAGONALS.contains(&diag2) {
                count += 1;
            }
        }
    }

    println!("{count}");
}
