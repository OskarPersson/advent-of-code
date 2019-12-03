use std::collections::HashMap;
use std::fs;


fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("part1: {}, part2: {}", part1(&contents), part2(&contents));
}

fn part1(contents: &String) -> i32{
    let mut total_twos = 0;
    let mut total_threes = 0;

    for line in contents.lines() {
        let (twos, threes) = get_counts(&String::from(line));
        total_twos += twos;
        total_threes += threes;
    }

    return total_twos * total_threes;
}

fn get_counts(s: &String) -> (i32, i32) {
    let mut twos = 0;
    let mut threes = 0;
    let mut map = HashMap::new();

    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    for (_, value) in map {
        if value == 2 {
            twos = 1
        }
        else if value == 3 {
            threes = 1
        }
    }

    return (twos, threes);
}

fn part2(contents: &String) -> String{
    let mut seen: Vec<String> = Vec::new();
    let mut same: String = String::from("");

    for line in contents.lines() {
        for old in &seen{
            if old.len() != line.len() {
                continue;
            }
            same = String::from("");
            for (c1, c2) in line.chars().zip(old.chars()) {
                if c1 == c2 {
                    same.push(c1)
                }
            }
            if same.len() == line.len() - 1 {
                return same;
            }
        }
        seen.push(String::from(line));
    }
    return same
}


