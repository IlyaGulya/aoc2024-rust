use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::TOP => Direction::RIGHT,
            Direction::RIGHT => Direction::BOTTOM,
            Direction::BOTTOM => Direction::LEFT,
            Direction::LEFT => Direction::TOP,
        }
    }

    fn go(&self, (i, j): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::TOP => (i - 1, j),
            Direction::RIGHT => (i, j + 1),
            Direction::BOTTOM => (i + 1, j),
            Direction::LEFT => (i, j - 1),
        }
    }
}

enum MapItem {
    Empty,
    Obstacle,
    Guard(Direction),
}

fn main() {
    let map = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => MapItem::Empty,
                    '#' => MapItem::Obstacle,
                    c => MapItem::Guard(match c {
                        '^' => Direction::TOP,
                        '>' => Direction::RIGHT,
                        'v' => Direction::BOTTOM,
                        '<' => Direction::LEFT,
                        _ => unreachable!(),
                    }),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max = ((map.len() - 1) as isize, (map[0].len() - 1) as isize);
    let (mut pos, mut direction) = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter().enumerate().find_map(|(j, item)| match item {
                MapItem::Guard(dir) => Some(((i as isize, j as isize), *dir)),
                _ => None,
            })
        })
        .expect("No guard found");

    let mut visited = HashSet::from([pos]);
    let mut next;

    loop {
        next = direction.go(pos);
        if next.0 < 0 || next.0 > max.0 || next.1 < 0 || next.1 > max.1 {
            break;
        }

        if matches!(map[next.0 as usize][next.1 as usize], MapItem::Obstacle) {
            direction = direction.turn_right();
        } else {
            pos = next;
            visited.insert(pos);
        }
    }

    println!("Visited {} positions", visited.len());
}
