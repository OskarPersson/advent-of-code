use core::panic;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

type SeatGrid = Vec<Vec<Position>>;

fn parse_input(input: &str) -> SeatGrid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|seat| match seat {
                    '.' => Position::Floor,
                    'L' => Position::Empty,
                    '#' => Position::Occupied,
                    _ => panic!("Unknown position type!"),
                })
                .collect::<Vec<Position>>()
        })
        .collect::<SeatGrid>()
}

fn is_occupied(pos: Option<Position>) -> bool {
    match pos {
        Some(Position::Occupied) => true,
        _ => false,
    }
}

fn get_pos(grid: &SeatGrid, i: i32, j: i32) -> Option<Position> {
    match grid.get(i as usize) {
        Some(row) => match row.get(j as usize) {
            Some(pos) => Some(*pos),
            _ => None,
        },
        None => None,
    }
}

fn occupied_adjacent_seats(grid: &SeatGrid, i: i32, j: i32) -> usize {
    let adjacent = vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ];

    adjacent
        .iter()
        .filter(|(i, j)| is_occupied(get_pos(grid, *i, *j)))
        .collect::<Vec<&(i32, i32)>>()
        .len()
}

fn simulate_round_part1(grid: SeatGrid) -> (SeatGrid, i32) {
    let mut changes = 0;
    let new = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, seat)| match seat {
                    Position::Empty => {
                        if occupied_adjacent_seats(&grid, i as i32, j as i32) == 0 {
                            changes += 1;
                            Position::Occupied
                        } else {
                            *seat
                        }
                    }
                    Position::Occupied => {
                        if occupied_adjacent_seats(&grid, i as i32, j as i32) >= 4 {
                            changes += 1;
                            Position::Empty
                        } else {
                            *seat
                        }
                    }
                    _ => *seat,
                })
                .collect::<Vec<Position>>()
        })
        .collect::<SeatGrid>();

    (new, changes)
}

fn part1(input: SeatGrid) -> usize {
    let (mut new, mut changes) = simulate_round_part1(input);

    while changes > 0 {
        let res = simulate_round_part1(new);
        new = res.0;
        changes = res.1;
    }

    new.into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|seat| *seat == Position::Occupied)
                .collect::<Vec<Position>>()
                .len()
        })
        .sum()
}

fn occupied_visible_seats(grid: &SeatGrid, i: i32, j: i32) -> usize {
    let mut count = 0;
    let mut new_i = i;
    let mut new_j = j;
    let mut pos = Some(Position::Floor);

    // up, left
    while pos == Some(Position::Floor) {
        new_i = new_i - 1;
        new_j = new_j - 1;
        pos = get_pos(grid, new_i, new_j);
    }
    if is_occupied(pos) {
        count += 1;
    }

    // up
    pos = Some(Position::Floor);
    new_i = i;
    new_j = j;
    while pos == Some(Position::Floor) {
        new_i = new_i - 1;
        pos = get_pos(grid, new_i, new_j);
    }
    if is_occupied(pos) {
        count += 1;
    }

    // up, right
    pos = Some(Position::Floor);
    new_i = i;
    new_j = j;
    while pos == Some(Position::Floor) {
        new_i = new_i - 1;
        new_j = new_j + 1;
        pos = get_pos(grid, new_i, new_j);
    }
    if is_occupied(pos) {
        count += 1;
    }

    // left
    pos = Some(Position::Floor);
    new_i = i;
    new_j = j;
    while pos == Some(Position::Floor) {
        new_j = new_j - 1;
        pos = get_pos(grid, new_i, new_j);
    }
    if is_occupied(pos) {
        count += 1;
    }

    // right
    pos = Some(Position::Floor);
    new_i = i;
    new_j = j;
    while pos == Some(Position::Floor) {
        new_j = new_j + 1;
        pos = get_pos(grid, new_i, new_j);
    }
    if is_occupied(pos) {
        count += 1;
    }

    // down, left
    pos = Some(Position::Floor);
    new_i = i;
    new_j = j;
    while pos == Some(Position::Floor) {
        new_i = new_i + 1;
        new_j = new_j - 1;
        pos = get_pos(grid, new_i, new_j);
    }
    if is_occupied(pos) {
        count += 1;
    }

    // down
    pos = Some(Position::Floor);
    new_i = i;
    new_j = j;
    while pos == Some(Position::Floor) {
        new_i = new_i + 1;
        pos = get_pos(grid, new_i, new_j);
    }
    if is_occupied(pos) {
        count += 1;
    }

    // down, right
    pos = Some(Position::Floor);
    new_i = i;
    new_j = j;
    while pos == Some(Position::Floor) {
        new_i = new_i + 1;
        new_j = new_j + 1;
        pos = get_pos(grid, new_i, new_j);
    }
    if is_occupied(pos) {
        count += 1;
    }

    count
    //todo!("Walk in each direction until finding either a Position::Empty or Position::Occupied seat, or walk off the grid")
}

fn simulate_round_part2(grid: SeatGrid) -> (SeatGrid, i32) {
    let mut changes = 0;
    let new = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, seat)| match seat {
                    Position::Empty => {
                        if occupied_visible_seats(&grid, i as i32, j as i32) == 0 {
                            changes += 1;
                            Position::Occupied
                        } else {
                            *seat
                        }
                    }
                    Position::Occupied => {
                        if occupied_visible_seats(&grid, i as i32, j as i32) >= 5 {
                            changes += 1;
                            Position::Empty
                        } else {
                            *seat
                        }
                    }
                    _ => *seat,
                })
                .collect::<Vec<Position>>()
        })
        .collect::<SeatGrid>();

    (new, changes)
}

fn part2(input: SeatGrid) -> usize {
    let (mut new, mut changes) = simulate_round_part2(input);

    while changes > 0 {
        let res = simulate_round_part2(new);
        new = res.0;
        changes = res.1;
    }

    new.into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|seat| *seat == Position::Occupied)
                .collect::<Vec<Position>>()
                .len()
        })
        .sum()
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let part1 = part1(parse_input(&contents));
    println!("part1: {}", part1);

    let part2 = part2(parse_input(&contents));
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1, part2};

    #[test]
    fn test_part1_example() {
        let input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        assert_eq!(part1(parse_input(input)), 37);
    }

    #[test]
    fn test_part2_example() {
        let input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        assert_eq!(part2(parse_input(input)), 26);
    }
}
