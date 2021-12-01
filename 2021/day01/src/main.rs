fn part1(entries: &Vec<i32>) -> i32 {
    let mut increases = 0;
    for slice in entries.windows(2) {
        if slice[1] > slice[0] {
            increases += 1;
        }
    }
    increases
}

fn part2(entries: &Vec<i32>) -> i32 {
    let mut increases = 0;
    for slice in entries
        .windows(3)
        .map(|x| x.into_iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
    {
        if slice[1] > slice[0] {
            increases += 1;
        }
    }

    increases
}

fn main() {
    let contents = include_str!("../input.txt");
    let entries = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let part1 = part1(&entries);
    println!("part1: {}", part1);

    let part2 = part2(&entries);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(
            part1(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
            7
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part2(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
            5
        );
    }
}
