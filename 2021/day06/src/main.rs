fn parse_input(contents: &str) -> Vec<usize> {
    contents
        .trim()
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

fn part1(contents: &str) -> i32 {
    let mut fish_list = parse_input(contents);

    for _ in 0..80 {
        let mut new_count = 0;
        for fish in fish_list.iter_mut() {
            match fish {
                0 => {
                    *fish = 6;
                    new_count += 1;
                }
                _ => *fish -= 1,
            }
        }
        for _ in 0..new_count {
            fish_list.push(8);
        }
    }

    fish_list.len() as i32
}

fn part2(contents: &str) -> i64 {
    let fish_list = parse_input(contents);

    let mut zeroes = fish_list.iter().filter(|f| **f == 0).count() as i64;
    let mut ones = fish_list.iter().filter(|f| **f == 1).count() as i64;
    let mut twos = fish_list.iter().filter(|f| **f == 2).count() as i64;
    let mut threes = fish_list.iter().filter(|f| **f == 3).count() as i64;
    let mut fours = fish_list.iter().filter(|f| **f == 4).count() as i64;
    let mut fives = fish_list.iter().filter(|f| **f == 5).count() as i64;
    let mut sixes = fish_list.iter().filter(|f| **f == 6).count() as i64;
    let mut sevens = fish_list.iter().filter(|f| **f == 7).count() as i64;
    let mut eights = fish_list.iter().filter(|f| **f == 8).count() as i64;

    for _ in 0..256 {
        let new_eights = zeroes;
        let new_sixes = zeroes;

        zeroes = ones;
        ones = twos;
        twos = threes;
        threes = fours;
        fours = fives;
        fives = sixes;
        sixes = sevens;
        sevens = eights;
        eights = new_eights;
        sixes += new_sixes;
    }

    [
        zeroes, ones, twos, threes, fours, fives, sixes, sevens, eights,
    ]
    .iter()
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
        let contents = "3,4,3,1,2";
        assert_eq!(part1(contents), 5934);
    }

    #[test]
    fn test_part_2() {
        let contents = "3,4,3,1,2";
        assert_eq!(part2(contents), 26984457539);
    }
}
