use regex::Regex;

fn main() {
    let contents = include_str!("../input.txt");
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]*)$").unwrap();
    let part1 = contents
        .lines()
        .filter_map(|line| {
            let caps = re.captures(line).unwrap();
            let minimum = caps[1].parse::<usize>().unwrap();
            let maximum = caps[2].parse::<usize>().unwrap();
            let character = &caps[3].chars().nth(0).unwrap();
            let password = &caps[4];
            let occurences = password
                .chars()
                .filter(|c| c == character)
                .collect::<Vec<char>>()
                .len();
            if minimum <= occurences && occurences <= maximum {
                return Some(line);
            }
            None
        })
        .collect::<Vec<&str>>()
        .len();

    println!("part1: {}", part1);

    let part2 = contents
        .lines()
        .filter_map(|line| {
            let caps = re.captures(line).unwrap();
            let pos1 = caps[1].parse::<usize>().unwrap() - 1;
            let pos2 = caps[2].parse::<usize>().unwrap() - 1;
            let character = &caps[3].chars().nth(0).unwrap();
            let password = &caps[4];

            let pos1_match = &password.chars().nth(pos1).unwrap() == character;
            let pos2_match = &password.chars().nth(pos2).unwrap() == character;

            if (pos1_match && !pos2_match) || (!pos1_match && pos2_match) {
                return Some(line);
            }
            None
        })
        .collect::<Vec<&str>>()
        .len();

    println!("part2: {}", part2);
}
