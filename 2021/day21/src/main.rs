fn parse_input(contents: &str) -> (i32, i32) {
    let p1 = contents
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i32;
    let p2 = contents
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i32;
    (p1, p2)
}

fn part1(contents: &str) -> i32 {
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

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(contents);
    println!("part1: {}", part1);
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
}
