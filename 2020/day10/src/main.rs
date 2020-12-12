use std::{collections::HashMap, fs};

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn part1(input: &Vec<i64>) -> i64 {
    let mut input = input.clone();
    input.push(0);
    input.sort();

    let ones: i64 = input
        .windows(2)
        .filter(|x| x[0] + 1 == x[1])
        .collect::<Vec<&[i64]>>()
        .len() as i64;

    let threes: i64 = input
        .windows(2)
        .filter(|x| x[0] + 3 == x[1])
        .collect::<Vec<&[i64]>>()
        .len() as i64;

    ones * (threes + 1)
}

fn find_valid_next(so_far: i64, all: Vec<i64>, max: i64, cache: &mut HashMap<String, i64>) -> i64 {
    if max - so_far == 3 {
        return 1;
    }

    all.iter()
        .filter_map(|x| {
            let diff = *x - so_far;

            if diff < 0 || diff > 3 {
                return None;
            }

            let mut all = all.clone();
            all.retain(|y| y > x);

            let key = all
                .clone()
                .iter()
                .map(ToString::to_string)
                .collect::<String>();
            match cache.get(&key) {
                Some(v) => Some(*v),
                None => {
                    let res = find_valid_next(*x, all, max, cache);
                    cache.insert(key, res);
                    Some(res)
                }
            }
        })
        .sum()
}

fn part2(input: &Vec<i64>) -> i64 {
    let input = input.clone();
    let max = input.iter().max().unwrap() + 3;
    let cache: &mut HashMap<String, i64> = &mut HashMap::new();

    find_valid_next(0, input, max, cache)
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers = parse_input(&contents);

    let part1 = part1(&numbers);
    println!("part1: {}", part1);

    let part2 = part2(&numbers);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::{part1, part2};

    #[test]
    fn test_part1_example1() {
        let numbers = parse_input(
            &r#"16
10
15
5
1
11
7
19
6
12
4"#,
        );
        assert_eq!(part1(&numbers), 7 * 5);
    }

    #[test]
    fn test_part1_example2() {
        let numbers = parse_input(
            &r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#,
        );
        assert_eq!(part1(&numbers), 22 * 10);
    }

    #[test]
    fn test_part2_example1() {
        let numbers = parse_input(
            &r#"16
10
15
5
1
11
7
19
6
12
4"#,
        );
        assert_eq!(part2(&numbers), 8);
    }

    #[test]
    fn test_part2_example2() {
        let numbers = parse_input(
            &r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#,
        );
        assert_eq!(part2(&numbers), 19208);
    }
}
