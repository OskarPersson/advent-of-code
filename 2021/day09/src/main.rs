use std::collections::HashMap;

fn parse_input(contents: &str) -> HashMap<i32, HashMap<i32, i32>> {
    let mut map: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    for (x, l) in contents.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            let d = c.to_digit(10).unwrap() as i32;
            if let Some(e) = map.get_mut(&(x as i32)) {
                e.insert(y as i32, d);
            } else {
                let mut xmap: HashMap<i32, i32> = HashMap::new();
                xmap.insert(y as i32, d);
                map.insert(x as i32, xmap);
            }
        }
    }
    map
}

fn part1(contents: &str) -> i32 {
    let entries = parse_input(contents);
    let mut risk_level = 0;

    for (i, x) in &entries {
        for (j, y) in x {
            if let Some(v) = entries.get(&i).unwrap().get(&(j + 1)) {
                if y >= v {
                    continue;
                }
            }
            if let Some(v) = entries.get(&i).unwrap().get(&(j - 1)) {
                if y >= v {
                    continue;
                }
            }
            if let Some(Some(v)) = entries.get(&(i + 1)).map(|v| v.get(&j)) {
                if y >= v {
                    continue;
                }
            }
            if let Some(Some(v)) = entries.get(&(i + 1)).map(|v| v.get(&(j + 1))) {
                if y >= v {
                    continue;
                }
            }
            if let Some(Some(v)) = entries.get(&(i + 1)).map(|v| v.get(&(j - 1))) {
                if y >= v {
                    continue;
                }
            }
            if let Some(Some(v)) = entries.get(&(i - 1)).map(|v| v.get(&j)) {
                if y >= v {
                    continue;
                }
            }
            if let Some(Some(v)) = entries.get(&(i - 1)).map(|v| v.get(&(j + 1))) {
                if y >= v {
                    continue;
                }
            }
            if let Some(Some(v)) = entries.get(&(i - 1)).map(|v| v.get(&(j - 1))) {
                if y >= v {
                    continue;
                }
            }
            risk_level += y + 1;
        }
    }

    risk_level
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(contents);
    println!("part1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let contents = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(part1(contents), 15);
    }
}
