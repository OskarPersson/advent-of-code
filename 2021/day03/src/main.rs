fn part1(entries: &Vec<&str>) -> i32 {
    let str_len = entries[0].len();
    let mut final_str = String::from("");
    let mut final_str_inverse = String::from("");

    for idx in 0..str_len {
        let mut zeros = 0;
        let mut ones = 0;
        for e in entries {
            if e.chars().nth(idx).unwrap() == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        if zeros > ones {
            final_str.push('0');
            final_str_inverse.push('1');
        } else {
            final_str.push('1');
            final_str_inverse.push('0');
        }
    }

    let gamma = i32::from_str_radix(&final_str, 2).unwrap();
    let epsilon = i32::from_str_radix(&final_str_inverse, 2).unwrap();

    gamma * epsilon
}

fn part2(entries: &Vec<&str>) -> i32 {
    let str_len = entries[0].len();
    let mut final_str = String::from("");
    let mut final_str_inverse = String::from("");

    for idx in 0..str_len {
        let mut zeros = 0;
        let mut ones = 0;
        for e in entries {
            if e.chars().nth(idx).unwrap() == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        if zeros > ones {
            final_str.push('0');
            final_str_inverse.push('1');
        } else {
            final_str.push('1');
            final_str_inverse.push('0');
        }
    }

    let gamma = i32::from_str_radix(&final_str, 2).unwrap();
    let epsilon = i32::from_str_radix(&final_str_inverse, 2).unwrap();

    gamma * epsilon
}

fn main() {
    let contents = include_str!("../input.txt");
    let entries = contents.lines().collect();

    let part1 = part1(&entries);
    println!("part1: {}", part1);

    let part2 = part2(&entries);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(
            part1(&vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]),
            198
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part2(&vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]),
            230
        );
    }
}
