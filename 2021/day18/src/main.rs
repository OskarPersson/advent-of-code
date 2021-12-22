use regex::{Captures, Regex};
use serde_json::Value;

fn parse_input(contents: &str) -> Vec<Value> {
    contents
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn magnitude(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(vv) => magnitude(&vv[0]) * 3 + magnitude(&vv[1]) * 2,
        _ => unreachable!(),
    }
}

fn explode(v: &mut Value) -> Value {
    let mut s = v.to_string();
    let mut level = 0;

    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        if c == '[' {
            level += 1;
        }
        if c == ']' {
            level -= 1;
        }
        if level < 5 || c != '[' {
            continue;
        }

        let substr = s
            .chars()
            .skip(i + 1)
            .take_while(|c| *c != ']')
            .collect::<String>();
        let re = Regex::new(r"^(\d+),(\d+)$").unwrap();
        if !re.is_match(&substr) {
            continue;
        }

        let pair = substr.split_once(',').unwrap();
        let pair = (
            pair.0.parse::<i32>().unwrap(),
            pair.1.parse::<i32>().unwrap(),
        );

        let regular_re = Regex::new(r"\d+").unwrap();

        let before_s = s.chars().take(i).collect::<String>();
        let before_s = regular_re
            .replace(
                &before_s.chars().rev().collect::<String>(),
                |caps: &Captures| {
                    let c2 = format!(
                        "{}",
                        caps[0]
                            .chars()
                            .rev()
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap()
                            + pair.0
                    );
                    format!("{}", c2.chars().rev().collect::<String>())
                },
            )
            .to_string()
            .chars()
            .rev()
            .collect::<String>();

        let after_s = s.chars().skip(i + substr.len() + 2).collect::<String>();
        let after_s = regular_re
            .replace(&after_s, |caps: &Captures| {
                format!("{}", caps[0].parse::<i32>().unwrap() + pair.1,)
            })
            .to_string();

        s = format!("{}0{}", before_s, after_s);
        break;
    }
    serde_json::from_str(&s).unwrap()
}

fn split(v: &mut Value) -> Value {
    let mut s = v.to_string();
    let re = Regex::new(r"(\d{2,})").unwrap();
    s = re
        .replace(&s, |caps: &Captures| {
            let n = caps[1].parse::<i32>().unwrap();
            format!("[{},{}]", n / 2, (n + 1) / 2)
        })
        .to_string();

    serde_json::from_str(&s).unwrap()
}

fn reduce(v: &Value) -> Value {
    let mut prev = v.clone();
    let mut new = explode(&mut prev);
    if new == prev {
        new = split(&mut prev)
    }

    while new != prev {
        prev = new;
        new = explode(&mut prev);
        if new == prev {
            new = split(&mut prev)
        }
    }

    new.clone()
}

fn add(a: &Value, b: &Value) -> Value {
    reduce(&Value::Array(vec![a.clone(), b.clone()]))
}

fn part1(contents: &str) -> i64 {
    let values = parse_input(contents);
    let mut values_iter = values.iter();

    let mut sum = add(values_iter.next().unwrap(), values_iter.next().unwrap());
    while let Some(v) = values_iter.next() {
        sum = add(&sum, v);
    }

    magnitude(&sum)
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
    fn test_magnitude_1() {
        assert_eq!(magnitude(&parse_input("[[1,2],[[3,4],5]]")[0]), 143);
    }

    #[test]
    fn test_magnitude_2() {
        assert_eq!(
            magnitude(&parse_input("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")[0]),
            1384
        );
    }

    #[test]
    fn test_magnitude_3() {
        assert_eq!(
            magnitude(&parse_input("[[[[1,1],[2,2]],[3,3]],[4,4]]")[0]),
            445
        );
    }

    #[test]
    fn test_magnitude_4() {
        assert_eq!(
            magnitude(&parse_input("[[[[3,0],[5,3]],[4,4]],[5,5]]")[0]),
            791
        );
    }

    #[test]
    fn test_magnitude_5() {
        assert_eq!(
            magnitude(&parse_input("[[[[5,0],[7,4]],[5,5]],[6,6]]")[0]),
            1137
        );
    }

    #[test]
    fn test_magnitude_6() {
        assert_eq!(
            magnitude(&parse_input("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")[0]),
            3488
        );
    }

    #[test]
    fn test_explode_1() {
        assert_eq!(
            explode(&mut parse_input("[[[[[9,8],1],2],3],4]")[0]),
            parse_input("[[[[0,9],2],3],4]")[0]
        );
    }

    #[test]
    fn test_explode_2() {
        assert_eq!(
            explode(&mut parse_input("[7,[6,[5,[4,[3,2]]]]]")[0]),
            parse_input("[7,[6,[5,[7,0]]]]")[0]
        );
    }

    #[test]
    fn test_explode_3() {
        assert_eq!(
            explode(&mut parse_input("[[6,[5,[4,[3,2]]]],1]")[0]),
            parse_input("[[6,[5,[7,0]]],3]")[0]
        );
    }

    #[test]
    fn test_explode_4() {
        assert_eq!(
            explode(&mut parse_input("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")[0]),
            parse_input("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")[0]
        );
    }

    #[test]
    fn test_explode_5() {
        assert_eq!(
            explode(&mut parse_input("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")[0]),
            parse_input("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")[0]
        );
    }

    #[test]
    fn test_explode_6() {
        assert_eq!(
            explode(&mut parse_input("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]")[0]),
            parse_input("[[[[0,7],4],[15,[0,13]]],[1,1]]")[0]
        );
    }

    #[test]
    fn test_explode_7() {
        assert_eq!(
            explode(&mut parse_input("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")[0]).to_string(),
            parse_input("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")[0].to_string()
        )
    }

    #[test]
    fn test_explode_8() {
        assert_eq!(
            explode(&mut parse_input("[[[[12,12],[6,14]],[[15,0],[17,[8,1]]]],[2,9]]")[0])
                .to_string(),
            parse_input("[[[[12,12],[6,14]],[[15,0],[25,0]]],[3,9]]")[0].to_string()
        )
    }

    #[test]
    fn test_split_1() {
        assert_eq!(
            split(&mut parse_input("[[[[0,7],4],[15,[0,13]]],[1,1]]")[0]),
            parse_input("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")[0]
        );
    }

    #[test]
    fn test_split_2() {
        assert_eq!(
            split(&mut parse_input("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")[0]),
            parse_input("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")[0]
        );
    }

    #[test]
    fn test_add_1() {
        assert_eq!(
            add(&parse_input("[1,2]")[0], &parse_input("[[3,4],5]")[0],),
            parse_input("[[1,2],[[3,4],5]]")[0]
        );
    }

    #[test]
    fn test_add_2() {
        assert_eq!(
            add(
                &parse_input("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")[0],
                &parse_input("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]")[0],
            ),
            parse_input("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]")[0]
        );
    }

    #[test]
    fn test_add_3() {
        assert_eq!(
            add(
                &parse_input("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]")[0],
                &parse_input("[2,9]")[0],
            ),
            parse_input("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]")[0]
        );
    }

    #[test]
    fn test_reduce_1() {
        assert_eq!(
            reduce(&parse_input("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")[0]),
            parse_input("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")[0]
        );
    }

    #[test]
    fn test_part_1_example_1() {
        assert_eq!(
            part1(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
            ),
            3488
        );
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(
            part1(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            ),
            4140
        );
    }
}
