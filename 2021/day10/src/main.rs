fn parse_input(contents: &str) -> Vec<&str> {
    contents.lines().collect()
}

fn is_opening(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn matching_char(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn corrupted_char_points(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn corrupted_char(line: &str) -> Option<char> {
    let mut opening = vec![];

    for c in line.chars() {
        if is_opening(c) {
            opening.push(c)
        } else {
            if c != matching_char(opening.pop().unwrap()) {
                return Some(c);
            }
        }
    }
    None
}

fn part1(contents: &str) -> i32 {
    let lines = parse_input(contents);
    lines
        .iter()
        .map(|l| {
            if let Some(c) = corrupted_char(l) {
                corrupted_char_points(c)
            } else {
                0
            }
        })
        .sum()
}

fn incomplete_char_points(c: char) -> i32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn incomplete_line(line: &str) -> Vec<char> {
    let mut opening = vec![];

    for c in line.chars() {
        if is_opening(c) {
            opening.push(c)
        } else {
            opening.pop();
        }
    }

    opening.reverse();
    opening.iter().map(|c| matching_char(*c)).collect()
}

fn part2(contents: &str) -> i64 {
    let lines = parse_input(contents);
    let incomplete: Vec<&str> = lines
        .into_iter()
        .filter(|l| match corrupted_char(l) {
            Some(_) => false,
            None => true,
        })
        .collect();

    let mut all_scores = vec![];
    for i in incomplete {
        let mut total: i64 = 0;
        for c in incomplete_line(i) {
            total = (total * 5) + incomplete_char_points(c) as i64;
        }
        all_scores.push(total);
    }

    all_scores.sort();
    *all_scores.get(all_scores.len() / 2).unwrap()
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
    fn test_corrupted_char_against_corrupted_line() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(corrupted_char(line), Some('}'));
    }

    #[test]
    fn test_corrupted_char_against_incomplete_line() {
        let line = "[({(<(())[]>[[{[]{<()<>>";
        assert_eq!(corrupted_char(line), None);
    }

    #[test]
    fn test_incomplete_line() {
        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(incomplete_line(line), vec![']', ')', '}', '>']);
    }

    #[test]
    fn test_part_1() {
        let contents = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(part1(contents), 26397);
    }

    #[test]
    fn test_part_2() {
        let contents = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(part2(contents), 288957);
    }
}
