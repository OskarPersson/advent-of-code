fn parse_input(contents: &str) -> Vec<i32> {
    contents
        .trim()
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}

fn part1(contents: &str) -> i32 {
    let crab_pos = parse_input(contents);
    let mut lowest_cost: Option<i32> = None;

    let min_pos = *crab_pos.iter().min().unwrap();
    let max_pos = *crab_pos.iter().max().unwrap();

    for target in min_pos..=max_pos {
        let mut target_cost = 0;
        for pos in &crab_pos {
            target_cost += (target - pos).abs()
        }
        if let Some(lowest) = lowest_cost {
            lowest_cost = Some(lowest.min(target_cost));
        } else {
            lowest_cost = Some(target_cost);
        }
    }
    lowest_cost.unwrap()
}

fn part2(contents: &str) -> i32 {
    let crab_pos = parse_input(contents);
    let mut lowest_cost: Option<i32> = None;

    let min_pos = *crab_pos.iter().min().unwrap();
    let max_pos = *crab_pos.iter().max().unwrap();

    for target in min_pos..=max_pos {
        let mut target_cost = 0;
        for pos in &crab_pos {
            target_cost += (1..=(target - pos).abs()).sum::<i32>()
        }
        if let Some(lowest) = lowest_cost {
            lowest_cost = Some(lowest.min(target_cost));
        } else {
            lowest_cost = Some(target_cost);
        }
    }
    lowest_cost.unwrap()
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
        let contents = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(part1(contents), 37);
    }

    #[test]
    fn test_part_2() {
        let contents = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(part2(contents), 168);
    }
}
