use std::fs;

fn find_invalid_number(numbers: &Vec<usize>, preamble_length: usize) -> (usize, usize) {
    let (idx, number) = numbers
        .iter()
        .enumerate()
        .skip(preamble_length)
        .find(|(idx, x)| {
            let start = idx - preamble_length;
            for (i_idx, i) in numbers[start..=idx - 2].iter().enumerate() {
                for j in numbers[start + i_idx + 1..=idx - 1].iter() {
                    if i + j == **x {
                        return false;
                    }
                }
            }

            return true;
        })
        .unwrap();
    (idx, *number)
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

fn part1(input: &Vec<usize>, preamble_length: usize) -> usize {
    find_invalid_number(input, preamble_length).1
}

fn part2(input: &Vec<usize>, preamble_length: usize) -> usize {
    let (target_idx, target) = find_invalid_number(input, preamble_length);

    for start in 0..target_idx - 1 {
        if start > target {
            continue;
        }
        for end in start + 1..target_idx {
            let slice = &input[start..=end];
            let sum = slice.iter().sum::<usize>();
            if sum == target {
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return min + max;
            } else if sum > target {
                break;
            }
        }
    }
    return 0;
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers = parse_input(&contents);

    let part1 = part1(&numbers, 25);
    println!("part1: {}", part1);

    let part2 = part2(&numbers, 25);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let numbers = parse_input(
            &r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#,
        );
        assert_eq!(part1(&numbers, 5), 127);
    }

    #[test]
    fn test_part2() {
        let numbers = parse_input(
            &r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#,
        );
        assert_eq!(part2(&numbers, 5), 62);
    }
}
