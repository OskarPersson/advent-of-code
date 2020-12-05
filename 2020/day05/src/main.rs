use core::panic;
use std::fs;

use num::integer::Integer;

fn calc_row(seat: &str) -> usize {
    let mut row = (0, 127);
    for row_char in seat.chars().take(7) {
        row = match row_char {
            'F' => (row.0, (row.0 + row.1).div_floor(&2)),
            'B' => ((row.0 + row.1).div_ceil(&2), row.1),
            _ => panic!("Invalid row character: {}", row_char),
        };
    }

    let row_char = seat.chars().nth(6).unwrap();
    match row_char {
        'F' => row.0,
        'B' => row.1,
        _ => panic!("Invalid row character: {}", row_char),
    }
}

fn calc_col(seat: &str) -> usize {
    let mut col = (0, 7);
    for col_char in seat.chars().skip(7) {
        col = match col_char {
            'L' => (col.0, (col.0 + col.1).div_floor(&2)),
            'R' => ((col.0 + col.1).div_ceil(&2), col.1),
            _ => panic!("Invalid col character: {}", col_char),
        }
    }

    let col_char = seat.chars().nth(9).unwrap();
    match col_char {
        'L' => col.0,
        'R' => col.1,
        _ => panic!("Invalid col character: {}", col_char),
    }
}

fn calc_seat_id(seat: &str) -> usize {
    let row = calc_row(seat);
    let col = calc_col(seat);

    (row * 8) + col
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let part1 = contents.lines().map(calc_seat_id).max().unwrap();
    println!("part1: {}", part1);

    let mut part2 = None;
    let ids = contents.lines().map(calc_seat_id).collect::<Vec<usize>>();
    for id in &ids {
        if (&ids).contains(&(id - 2)) && !(&ids).contains(&(id - 1)) {
            part2 = Some(id - 1);
            break;
        }

        if (&ids).contains(&(id + 2)) && !(&ids).contains(&(id + 1)) {
            part2 = Some(id + 1);
            break;
        }
    }

    println!("part2: {}", part2.unwrap());
}

#[cfg(test)]
mod tests {
    use super::calc_seat_id;

    #[test]
    fn test_example_1() {
        assert_eq!(calc_seat_id("FBFBBFFRLR"), 357);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(calc_seat_id("BFFFBBFRRR"), 567);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(calc_seat_id("FFFBBBFRRR"), 119);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(calc_seat_id("BBFFBBFRLL"), 820);
    }
}
