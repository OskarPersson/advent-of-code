use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim_end()
        .to_string();

    let part1 = react(&contents);
    let part2 = part2(&contents);

    println!("part 1: {}, part 2: {}", part1, part2);
}

fn react(contents: &String) -> usize {
    let mut results = contents.clone();
    let mut prev: Option<char> = None;
    let mut changed = true;

    while changed {
        changed = false;
        for (idx, c) in results.chars().enumerate() {
            if let Some(p) = prev {
                if p.to_uppercase().to_string() == c.to_uppercase().to_string() {
                    if p != c {
                        results.remove(idx - 1);
                        results.remove(idx - 1);
                        changed = true;
                        break;
                    }
                }
            }
            prev = Some(c);
        }
        prev = None;
    }
    return results.chars().count();
}

fn part2(contents: &String) -> usize {
    let alphabet = b'a'..b'z' + 1;
    let r = alphabet.map(|c| {
            let c = c as char;
            let mut s = contents.clone();
            s.retain(|sc| sc != c && sc != c.to_ascii_uppercase());
            return react(&s);
        }).min().unwrap();

    return r;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let unit_count = part1(&String::from("aA"));
        assert_eq!(unit_count, 0);

        let unit_count = part1(&String::from("abBA"));
        assert_eq!(unit_count, 0);

        let unit_count = part1(&String::from("abAB"));
        assert_eq!(unit_count, 4);

        let unit_count = part1(&String::from("aabAAB"));
        assert_eq!(unit_count, 6);

        let unit_count = part1(&String::from("dabAcCaCBAcCcaDA"));
        assert_eq!(unit_count, 10);

        let unit_count = part1(&String::from("dabCBAcaDA"));
        assert_eq!(unit_count, 10);
    }
}
