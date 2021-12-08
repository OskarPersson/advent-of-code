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
}
