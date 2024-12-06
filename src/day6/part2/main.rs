#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next_position(self, (row, col): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (row.wrapping_sub(1), col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col.wrapping_sub(1)),
        }
    }

    fn to_id(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

struct Map {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(input: &str) -> (Self, (usize, usize), Direction) {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let rows = grid.len();
        let cols = grid[0].len();

        // Find guard's starting position and direction
        let (start_pos, start_dir) = (0..rows)
            .flat_map(|r| (0..cols).map(move |c| (r, c)))
            .find_map(|(r, c)| match grid[r][c] {
                '^' => Some(((r, c), Direction::Up)),
                'v' => Some(((r, c), Direction::Down)),
                '<' => Some(((r, c), Direction::Left)),
                '>' => Some(((r, c), Direction::Right)),
                _ => None,
            })
            .unwrap();

        (Map { grid, rows, cols }, start_pos, start_dir)
    }

    fn is_valid_pos(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.rows && pos.1 < self.cols
    }
}

fn to_state_id(row: usize, col: usize, cols: usize, dir: Direction) -> usize {
    (row * cols + col) * 4 + dir.to_id()
}

fn has_loop(
    map: &Map,
    start_pos: (usize, usize),
    start_dir: Direction,
    obstruction: Option<(usize, usize)>,
) -> bool {
    let mut visited = vec![false; map.rows * map.cols * 4];
    let mut pos = start_pos;
    let mut dir = start_dir;

    // Set initial state as visited
    if map.is_valid_pos(pos) {
        visited[to_state_id(pos.0, pos.1, map.cols, dir)] = true;
    }

    for _ in 0..visited.len() {
        let next_pos = dir.next_position(pos);

        // Check if next position is valid and not blocked
        let blocked = if map.is_valid_pos(next_pos) {
            let cell = map.grid[next_pos.0][next_pos.1];
            obstruction.map_or(cell == '#', |obs| obs == next_pos || cell == '#')
        } else {
            return false; // Guard left the map
        };

        if blocked {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
        }

        if map.is_valid_pos(pos) {
            let state_id = to_state_id(pos.0, pos.1, map.cols, dir);
            if visited[state_id] {
                return true; // Loop found
            }
            visited[state_id] = true;
        } else {
            return false; // Guard left the map
        }
    }
    false
}

fn main() {
    let input = include_str!("../input.txt");
    let (map, start_pos, start_dir) = Map::new(input);

    // Count '.' positions for progress tracking
    let total_candidates = map
        .grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == '.')
        .count()
        - 1; // Subtract 1 for start position

    let mut count = 0;
    let mut tested = 0;

    for r in 0..map.rows {
        for c in 0..map.cols {
            if (r, c) != start_pos && map.grid[r][c] == '.' {
                if has_loop(&map, start_pos, start_dir, Some((r, c))) {
                    count += 1;
                }

                tested += 1;
                if tested % 1000 == 0 {
                    eprintln!("Tested {}/{} positions...", tested, total_candidates);
                }
            }
        }
    }

    println!("{}", count);
}
