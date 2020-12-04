use regex::Regex;
use std::fs;

fn part1(contents: &str) -> usize {
    contents
        .split("\n\n")
        .filter(|passport| {
            let fields: Vec<&str> = passport
                .split_whitespace()
                .map(|field| field.split(':').collect::<Vec<&str>>()[0])
                .collect();
            if fields.len() == 8 {
                return true;
            }

            fields.len() == 7 && !fields.contains(&"cid")
        })
        .collect::<Vec<&str>>()
        .len()
}

fn part2(contents: &str) -> usize {
    let hcl_re = Regex::new(r"^#[a-f\d]{6}$").unwrap();

    contents
        .split("\n\n")
        .filter(|passport| {
            let fields: Vec<&str> = passport
                .split_whitespace()
                .filter(|field| {
                    let f = field.split(':').collect::<Vec<&str>>();
                    let (k, v) = (f[0], f[1]);

                    match k {
                        "byr" => {
                            1920 <= v.parse::<i32>().unwrap() && v.parse::<i32>().unwrap() <= 2002
                        }
                        "iyr" => {
                            2010 <= v.parse::<i32>().unwrap() && v.parse::<i32>().unwrap() <= 2020
                        }
                        "eyr" => {
                            2020 <= v.parse::<i32>().unwrap() && v.parse::<i32>().unwrap() <= 2030
                        }
                        "hgt" => {
                            if v.ends_with("cm") {
                                let x = v.split('c').collect::<Vec<&str>>()[0]
                                    .parse::<i32>()
                                    .unwrap();
                                150 <= x && x <= 193
                            } else if v.ends_with("in") {
                                let x = v.split('i').collect::<Vec<&str>>()[0]
                                    .parse::<i32>()
                                    .unwrap();
                                59 <= x && x <= 76
                            } else {
                                false
                            }
                        }
                        "hcl" => hcl_re.is_match(v),
                        "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
                        "pid" => v.len() == 9 && v.parse::<i32>().is_ok(),
                        _ => false,
                    }
                })
                .collect();

            fields.len() == 7
        })
        .collect::<Vec<&str>>()
        .len()
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}
