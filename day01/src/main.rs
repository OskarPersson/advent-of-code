use std::collections::HashSet;
use std::fs;

fn get_frequencies(contents: &String, mut frequency: i32, frequencies: &mut HashSet<i32>) -> (i32, Option<i32>) {
    let mut reached_twice: Option<i32> = None;
    for line in contents.lines() {
        let new = line.parse::<i32>().unwrap();
        frequency += new;
        if frequencies.contains(&frequency) && reached_twice.is_none() {
            reached_twice = Some(frequency)
        } else {
            frequencies.insert(frequency);
        }
    }
    return (frequency, reached_twice);
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut frequency = 0;
    let mut frequencies: HashSet<i32> = HashSet::new();

    let (part1, mut part2) = get_frequencies(&contents, frequency, &mut frequencies);
    frequency = part1;

    while part2.is_none() {
        let (f, r) = get_frequencies(&contents, frequency, &mut frequencies);
        frequency = f;
        part2 = r;
    }

    print!("Part 1: {}, Part 2: {}", part1, part2.unwrap());
}
