fn part1(entries: &Vec<&str>) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for e in entries {
        let (direction, distance) = e.split_once(' ').unwrap();
        let distance = distance.parse::<i32>().unwrap();

        match direction {
            "forward" => x += distance,
            "down" => y += distance,
            "up" => y -= distance,
            _ => {}
        }
    }
    x * y
}

fn part2(entries: &Vec<&str>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for e in entries {
        let (direction, distance) = e.split_once(' ').unwrap();
        let distance = distance.parse::<i32>().unwrap();

        match direction {
            "forward" => {
                x += distance;
                y += aim * distance;
            }
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => {}
        }
    }
    x * y
}

fn main() {
    let contents = include_str!("../input.txt");
    let entries = contents.lines().collect();

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
            part1(&vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]),
            150
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part2(&vec![
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]),
            900
        );
    }
}
