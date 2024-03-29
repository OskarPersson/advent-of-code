fn parse_input(contents: &str) -> (Vec<Vec<char>>, String) {
    let algorithm = contents.lines().map(|l| l.to_string()).take(1).collect();
    let input = contents
        .lines()
        .skip(2)
        .map(|l| l.chars().collect())
        .collect();
    (input, algorithm)
}

fn get_algo_idx_for_pixel(input: &Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let mut s = String::from("");
    let edge_char = input[0][0];
    for x in [-1, 0, 1] {
        match i + x {
            n if n > -1 => {
                for y in [-1, 0, 1] {
                    match j + y {
                        n if n > -1 => {
                            let c = input.get((i + x) as usize).map_or(&edge_char, |l| {
                                l.get((j + y) as usize).unwrap_or(&edge_char)
                            });
                            s.push(match c {
                                '.' => '0',
                                '#' => '1',
                                _ => unreachable!(),
                            });
                        }
                        _ => {
                            s.push(match edge_char {
                                '.' => '0',
                                '#' => '1',
                                _ => unreachable!(),
                            });
                        }
                    }
                }
            }
            _ => {
                s.push(match edge_char {
                    '.' => '0',
                    '#' => '1',
                    _ => unreachable!(),
                });
            }
        }
    }
    i32::from_str_radix(&s, 2).unwrap()
}

fn enhance(input: &Vec<Vec<char>>, algorithm: &str) -> Vec<Vec<char>> {
    let input = extend(input, input[0][0]);
    let mut output = vec![];
    for i in 0..input.len() {
        let mut l = vec![];
        for j in 0..input[i].len() {
            let algo_idx = get_algo_idx_for_pixel(&input, i as i32, j as i32);
            l.push(algorithm.chars().nth(algo_idx as usize).unwrap());
        }
        output.push(l);
    }
    output
}

fn extend(input: &Vec<Vec<char>>, c: char) -> Vec<Vec<char>> {
    let line_len = input[0].len() + 2;
    let mut output = vec![vec![c; line_len]; 1];
    for i in 0..input.len() {
        let mut l = vec![];
        l.extend(input[i].iter());
        l.insert(0, c);
        l.push(c);
        output.push(l);
    }
    output.push(vec![c; line_len]);
    output
}

fn part1(contents: &str) -> i32 {
    let (input, algorithm) = parse_input(contents);
    let input = extend(&input, '.');
    let output = enhance(&enhance(&input, &algorithm), &algorithm);

    output
        .iter()
        .map(|l| l.iter().filter(|c| **c == '#').count() as i32)
        .sum()
}

fn part2(contents: &str) -> i32 {
    let (input, algorithm) = parse_input(contents);
    let input = extend(&input, '.');
    let mut output = input;
    for _ in 0..50 {
        output = enhance(&output, &algorithm);
    }

    output
        .iter()
        .map(|l| l.iter().filter(|c| **c == '#').count() as i32)
        .sum()
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
                "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
            ),
            35
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            part2(
                "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
            ),
            3351
        );
    }
}
