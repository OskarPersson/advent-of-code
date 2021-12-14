use std::collections::HashMap;

fn parse_input(contents: &str) -> (String, Vec<(&str, char)>) {
    let (template, rules) = contents.split_once("\n\n").unwrap();

    let rules: Vec<(&str, char)> = rules
        .lines()
        .map(|l| {
            let (pos, el) = l.split_once(" -> ").unwrap();
            (pos, el.chars().next().unwrap())
        })
        .collect();

    (template.to_string(), rules)
}

// divide and conquer?
// Split string in half, and for each half recursively:
// 1. check if template and rule combo already exists in cache
//  1a. if in template return from the cache
//  1b. otherwise, recursively check each half until we either find it in cache
//      or we get to two characters.
//      Then work our way backup from the recursiveness and merge each half and
//      put them in the cache

fn run_steps(contents: &str, steps: i32) -> i64 {
    let (mut template, rules) = parse_input(contents);
    let mut cache: HashMap<&str, &str> = HashMap::new();

    for step in 0..steps {
        for (idx, window) in template
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .enumerate()
        {
            let (_, chr) = rules
                .iter()
                .find(|r| r.0 == String::from_iter(window).as_str())
                .unwrap();
            template.insert(idx + (1 + (1 * idx)), *chr);
        }
        println!("Step {} --> {}", step + 1, template.len());
    }

    let mut map: HashMap<char, i64> = HashMap::new();
    for c in template.chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }

    let min = map.iter().min_by_key(|(_, v)| *v).unwrap().1;
    let max = map.iter().max_by_key(|(_, v)| *v).unwrap().1;
    max - min
}

fn part1(contents: &str) -> i64 {
    run_steps(contents, 10)
}

fn part2(contents: &str) -> i64 {
    run_steps(contents, 40)
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(contents);
    println!("part1: {}", part1);

    let part2 = part2(contents);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let contents = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(part1(contents), 1588);
    }

    #[test]
    fn test_part_2() {
        let contents = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(part2(contents), 2188189693529);
    }
}
