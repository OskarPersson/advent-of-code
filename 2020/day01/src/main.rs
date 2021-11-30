fn part1(sum: i32, entries: &Vec<i32>) -> Option<i32> {
    for (idx_i, i) in entries.iter().enumerate() {
        for j in entries.iter().skip(idx_i + 1) {
            if i + j == sum {
                return Some(*i * *j);
            }
        }
    }
    None
}

fn part2(sum: i32, entries: &Vec<i32>) -> Option<i32> {
    for (idx_i, i) in entries.iter().enumerate() {
        for (idx_j, j) in entries.iter().skip(idx_i + 1).enumerate() {
            for k in entries.iter().skip(idx_j + 1) {
                if i + j + k == sum {
                    return Some(*i * *j * *k);
                }
            }
        }
    }
    None
}

fn main() {
    let contents = include_str!("../input.txt");
    let entries = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let part1 = part1(2020, &entries);
    if let Some(part1) = part1 {
        println!("part1: {}", part1);
    }

    let part2 = part2(2020, &entries);
    if let Some(part2) = part2 {
        println!("part2: {}", part2);
    }
}
