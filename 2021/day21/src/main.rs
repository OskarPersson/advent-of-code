use std::collections::HashMap;

type Cache = HashMap<(i64, i64, i64, i64, bool), (i64, i64)>;

fn parse_input(contents: &str) -> (i64, i64) {
    let p1 = contents
        .lines()
        .next()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i64;
    let p2 = contents
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i64;
    (p1, p2)
}

fn part1(contents: &str) -> i64 {
    let (mut p1_space, mut p2_space) = parse_input(contents);
    let (mut p1_score, mut p2_score) = (0, 0);
    let mut die_throws = 0;

    loop {
        let steps = (die_throws + 1) + (die_throws + 2) + (die_throws + 3);
        die_throws += 3;

        p1_space = (p1_space + steps) % 10;
        if p1_space == 0 {
            p1_space = 10;
        }
        p1_score += p1_space;

        if p1_score >= 1000 {
            break;
        }

        let steps = (die_throws + 1) + (die_throws + 2) + (die_throws + 3);
        die_throws += 3;

        p2_space = (p2_space + steps) % 10;
        if p2_space == 0 {
            p2_space = 10;
        }
        p2_score += p2_space;

        if p2_score >= 1000 {
            break;
        }
    }

    p1_score.min(p2_score) * die_throws
}

fn quantum_roll(
    p1_space: i64,
    p1_score: i64,
    p2_space: i64,
    p2_score: i64,
    p1: bool,
    cache: &mut Cache,
) -> (i64, i64) {
    if p1_score >= 21 {
        return (1, 0);
    }
    if p2_score >= 21 {
        return (0, 1);
    }

    let cache_key = (p1_space, p1_score, p2_space, p2_score, p1);

    if let Some(e) = cache.get(&cache_key) {
        return *e;
    }

    let mut p1_total = 0;
    let mut p2_total = 0;

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let val = i + j + k;
                let (p1_wins, p2_wins) = if p1 {
                    let mut new_space = (p1_space + val) % 10;
                    if new_space == 0 {
                        new_space = 10;
                    }
                    quantum_roll(
                        new_space,
                        p1_score + new_space,
                        p2_space,
                        p2_score,
                        !p1,
                        cache,
                    )
                } else {
                    let mut new_space = (p2_space + val) % 10;
                    if new_space == 0 {
                        new_space = 10;
                    }
                    quantum_roll(
                        p1_space,
                        p1_score,
                        new_space,
                        p2_score + new_space,
                        !p1,
                        cache,
                    )
                };
                p1_total += p1_wins;
                p2_total += p2_wins;
            }
        }
    }

    cache.insert(cache_key, (p1_total, p2_total));

    (p1_total, p2_total)
}

fn part2(contents: &str) -> i64 {
    let (p1_space, p2_space) = parse_input(contents);
    let (p1_score, p2_score): (i64, i64) = (0, 0);

    let mut cache: Cache = HashMap::new();

    let (p1, p2) = quantum_roll(p1_space, p1_score, p2_space, p2_score, true, &mut cache);

    p1.max(p2)
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
        assert_eq!(
            part1(
                "Player 1 starting position: 4
Player 2 starting position: 8"
            ),
            739785
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part2(
                "Player 1 starting position: 4
Player 2 starting position: 8"
            ),
            444356092776315
        );
    }
}
