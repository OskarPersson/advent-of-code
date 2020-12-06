use std::collections::HashMap;
use std::fs;

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|grp| {
            let mut answers = grp
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .chars()
                .collect::<Vec<char>>();
            answers.sort();
            answers.dedup();
            answers.len()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|grp| {
            let grp_len = grp.lines().collect::<Vec<&str>>().len();
            let answers = grp
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .chars()
                .collect::<Vec<char>>();

            let mut map = HashMap::new();
            for a in answers {
                *map.entry(a).or_insert(0) += 1;
            }

            map.values()
                .filter(|v| **v == grp_len)
                .collect::<Vec<&usize>>()
                .len()
        })
        .sum()
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let part1 = part1(&contents);
    println!("part1: {}", part1);

    let part2 = part2(&contents);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let r = part1(
            r#"abc

a
b
c

ab
ac

a
a
a
a

b"#,
        );
        assert_eq!(r, 11);
    }

    #[test]
    fn test_part2() {
        let r = part2(
            r#"abc

a
b
c

ab
ac

a
a
a
a

b"#,
        );
        assert_eq!(r, 6);
    }
}
