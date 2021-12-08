fn parse_input(contents: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    contents
        .lines()
        .map(|l| {
            let mut l = l.split(" | ");
            let signal_patterns = l.next().unwrap();
            let output = l.next().unwrap();
            (
                signal_patterns.split(' ').collect(),
                output.split(' ').collect(),
            )
        })
        .collect()
}

fn part1(contents: &str) -> i32 {
    let entries = parse_input(contents);
    entries
        .into_iter()
        .map(|(_, output)| {
            output
                .iter()
                .map(|s| {
                    if s.chars().count() <= 4 || s.chars().count() == 7 {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn sort_chars(s: &str) -> String {
    let mut v = s.chars().collect::<Vec<char>>();
    v.sort_unstable();
    v.into_iter().collect()
}

fn extract_patterns<F>(patterns: &mut Vec<String>, filter: F) -> String
where
    F: FnMut(&String) -> bool,
{
    let idx = patterns.iter().position(filter).unwrap();
    patterns.remove(idx)
}

fn parse_line(signal_patterns: Vec<&str>, output: Vec<&str>) -> i32 {
    let mut signal_patterns = signal_patterns
        .iter()
        .map(|s| sort_chars(s))
        .collect::<Vec<String>>();

    let one = extract_patterns(&mut signal_patterns, |s: &String| s.chars().count() == 2);
    let four = extract_patterns(&mut signal_patterns, |s: &String| s.chars().count() == 4);
    let seven = extract_patterns(&mut signal_patterns, |s: &String| s.chars().count() == 3);
    let eight = extract_patterns(&mut signal_patterns, |s: &String| s.chars().count() == 7);
    let nine = extract_patterns(&mut signal_patterns, |s: &String| {
        s.chars().count() == 6
            && four
                .chars()
                .into_iter()
                .all(|item| s.chars().any(|x| x == item))
    });
    let zero = extract_patterns(&mut signal_patterns, |s: &String| {
        s.chars().count() == 6
            && one
                .chars()
                .into_iter()
                .all(|item| s.chars().any(|x| x == item))
    });
    let six = extract_patterns(&mut signal_patterns, |s: &String| s.chars().count() == 6);
    let three = extract_patterns(&mut signal_patterns, |s: &String| {
        s.chars().count() == 5
            && one
                .chars()
                .into_iter()
                .all(|item| s.chars().any(|x| x == item))
    });
    let five = extract_patterns(&mut signal_patterns, |s: &String| {
        s.chars().count() == 5
            && s.chars()
                .into_iter()
                .all(|item| nine.chars().any(|x| x == item))
    });
    let two = extract_patterns(&mut signal_patterns, |s: &String| s.chars().count() == 5);

    output
        .iter()
        .map(|s| sort_chars(s))
        .map(|s| {
            if s == zero {
                '0'
            } else if s == one {
                '1'
            } else if s == two {
                '2'
            } else if s == three {
                '3'
            } else if s == four {
                '4'
            } else if s == five {
                '5'
            } else if s == six {
                '6'
            } else if s == seven {
                '7'
            } else if s == eight {
                '8'
            } else if s == nine {
                '9'
            } else {
                unreachable!()
            }
        })
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

fn part2(contents: &str) -> i32 {
    let entries = parse_input(contents);
    entries
        .into_iter()
        .map(|(signal_patterns, output)| parse_line(signal_patterns, output))
        .sum::<i32>()
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
        let contents =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(part1(contents), 26);
    }

    #[test]
    fn test_part_2() {
        let contents =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(part2(contents), 61229);
    }

    #[test]
    fn test_parse_line() {
        let contents =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let (signal_patterns, output) = parse_input(contents).into_iter().next().unwrap();
        assert_eq!(parse_line(signal_patterns, output), 5353);
    }
}
