use std::collections::HashMap;

use regex::Regex;

fn parse_input(contents: &str) -> Vec<((i32, i32), (i32, i32))> {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    contents
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (
                (
                    caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                ),
                (
                    caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                ),
            )
        })
        .collect()
}

fn part1(contents: &str) -> i32 {
    let lines = parse_input(contents);
    let mut grid: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    for (src, dst) in lines {
        if src.1 == dst.1 {
            let x_src = src.0.min(dst.0);
            let x_dst = src.0.max(dst.0);
            for x in x_src..=x_dst {
                let e = grid
                    .entry(x)
                    .or_insert(HashMap::new())
                    .entry(src.1)
                    .or_insert(0);
                *e += 1;
            }
        } else if src.0 == dst.0 {
            let y_src = src.1.min(dst.1);
            let y_dst = src.1.max(dst.1);
            for y in y_src..=y_dst {
                let e = grid
                    .entry(src.0)
                    .or_insert(HashMap::new())
                    .entry(y)
                    .or_insert(0);
                *e += 1;
            }
        }
    }

    grid.into_iter()
        .map(|(_, x)| x.into_iter().filter(|(_, y)| *y > 1).count())
        .sum::<usize>() as i32
}

fn part2(contents: &str) -> i32 {
    let lines = parse_input(contents);
    let mut grid: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    for (src, dst) in lines {
        let x_range = if src.0 < dst.0 {
            (src.0..=dst.0).collect::<Vec<i32>>()
        } else {
            (dst.0..=src.0).rev().collect::<Vec<i32>>()
        };
        let x_len = x_range.len();

        let y_range = if src.1 < dst.1 {
            (src.1..=dst.1).collect::<Vec<i32>>()
        } else {
            (dst.1..=src.1).rev().collect::<Vec<i32>>()
        };
        let y_len = y_range.len();

        for i in 0..x_len.max(y_len) {
            let x = x_range.get(i).map_or(x_range.last().unwrap(), |c| c);
            let y = y_range.get(i).map_or(y_range.last().unwrap(), |c| c);
            let e = grid
                .entry(*x)
                .or_insert(HashMap::new())
                .entry(*y)
                .or_insert(0);
            *e += 1;
        }
    }

    grid.into_iter()
        .map(|(_, x)| x.into_iter().filter(|(_, y)| *y > 1).count())
        .sum::<usize>() as i32
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
        let contents = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(part1(contents), 5);
    }

    #[test]
    fn test_part_2() {
        let contents = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(part2(contents), 12);
    }
}
