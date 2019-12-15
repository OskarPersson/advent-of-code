use std::collections::HashMap;
use std::fs;

fn count_orbits(hm: &HashMap<&str, &str>, k: &str) -> i32 {
    if let Some(v) = hm.get(k) {
        return 1 + count_orbits(hm, v);
    }
    return 0;
}

fn get_orbits<'a>(hm: &HashMap<&str, &'a str>, k: &str, mut orbits: Vec<&'a str>) -> Vec<&'a str> {
    if let Some(v) = hm.get(k) {
        orbits.push(v);
        return get_orbits(hm, v, orbits);
    } else {
        return orbits;
    }
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("error!");

    let mut hm = HashMap::new();

    for line in contents.lines() {
        let split: Vec<&str> = line.split_terminator(")").collect();
        let first = split[0];
        let second = split[1];
        hm.insert(second, first);
    }

    let mut total = 0;

    for k in hm.keys() {
        total += count_orbits(&hm, k);
    }
    println!("part1: {}", total);

    // part 2

    let mut hm = HashMap::new();

    for line in contents.lines() {
        let split: Vec<&str> = line.split_terminator(")").collect();
        let first = split[0];
        let second = split[1];
        hm.insert(second, first);
    }

    let you_orbs = get_orbits(&hm, "YOU", vec![]);
    let san_orbs = get_orbits(&hm, "SAN", vec![]);

    let mut common: Option<&str> = None;
    let mut steps = 0;
    for (i, orb) in san_orbs.iter().enumerate() {
        if you_orbs.contains(orb) {
            steps = i;
            common = Some(orb);
            break;
        }
    }

    let you_pos = you_orbs.iter().position(|x| x == &common.unwrap()).unwrap();

    println!("part2: {}", steps + you_pos);
}
