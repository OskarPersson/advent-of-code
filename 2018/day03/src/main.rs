#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

struct Claim {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

fn parse(claim: &String) -> Claim {
    lazy_static! {
        static ref re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let cap = re.captures(claim).unwrap();
    return Claim {
        id: cap[1].parse::<i32>().unwrap(),
        left: cap[2].parse::<i32>().unwrap(),
        top: cap[3].parse::<i32>().unwrap(),
        width: cap[4].parse::<i32>().unwrap(),
        height: cap[5].parse::<i32>().unwrap(),
    }
}


fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut fabric: HashMap<String, Vec<i32>> = HashMap::new();
    let mut intact = vec![];

    for line in contents.lines() {
        let claim = parse(&String::from(line));
        intact.push(claim.id);

        let mut x = claim.left;
        while x < claim.left + claim.width {
            let mut y = claim.top;
            while y < claim.top + claim.height {
                let key = format!("{}x{}", x, y);
                let sqr_inch = fabric.entry(key).or_insert(vec![]);
                sqr_inch.push(claim.id);

                y += 1;
            }
            x += 1;
        }
    }

    let mut collision_count = 0;

    for (_, val) in fabric {
        if val.len() > 1 {
            collision_count += 1;
            for id in val {
                intact.retain(|&x| x != id)
            }
        }
    }

    println!("part 1: {}, part 2: {}", collision_count, intact[0]);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let claim = parse(&String::from("#1252 @ 506,743: 21x19"));
        assert_eq!(1252, claim.id);
        assert_eq!(506, claim.left);
        assert_eq!(743, claim.top);
        assert_eq!(21, claim.width);
        assert_eq!(19, claim.height);
    }
}
