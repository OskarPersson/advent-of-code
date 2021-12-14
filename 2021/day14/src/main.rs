use std::collections::HashMap;

fn parse_input(contents: &str) -> (Vec<String>, Vec<(&str, char)>) {
    let (template, rules) = contents.split_once("\n\n").unwrap();

    let rules: Vec<(&str, char)> = rules
        .lines()
        .map(|l| {
            let (pos, el) = l.split_once(" -> ").unwrap();
            (pos, el.chars().next().unwrap())
        })
        .collect();

    let f = template
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|w| String::from_iter(w))
        .collect();

    (f, rules)
}

fn run_steps(contents: &str, steps: i32) -> i64 {
    let (template, rules) = parse_input(contents);

    let mut counters: HashMap<String, i64> = HashMap::new();

    for t in &template {
        let e = counters.entry(t.clone()).or_insert(0);
        *e += 1;
    }

    for _ in 0..steps {
        let mut new_counters: HashMap<String, i64> = HashMap::new();

        for (pair, chr) in &rules {
            if counters.contains_key(*pair) {
                let n = counters.get(*pair).unwrap();
                let first = pair.chars().next().unwrap();
                let second = pair.chars().nth(1).unwrap();

                let first_str = String::from_iter([first, *chr]);
                let first = new_counters.entry(first_str.clone()).or_insert(0);
                *first += n;

                let second_str = String::from_iter([*chr, second]);
                let second = new_counters.entry(second_str.clone()).or_insert(0);
                *second += n;

                let entry = new_counters.entry(pair.to_string()).or_insert(0);
                *entry -= n;
            }
        }

        for (k, v) in new_counters {
            let entry = counters.entry(k.clone()).or_insert(0);
            *entry += v;
            if *entry <= 0 {
                counters.remove(&k);
            }
        }
    }

    let mut map: HashMap<char, i64> = HashMap::new();
    for (k, v) in &counters {
        for c in k.chars() {
            let counter = map.entry(c).or_insert(0);
            *counter += v;
        }
    }
    let min = (map.iter().min_by_key(|(_, v)| *v).unwrap().1 + 1) / 2;
    let max = (map.iter().max_by_key(|(_, v)| *v).unwrap().1 + 1) / 2;
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
