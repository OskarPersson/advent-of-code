fn parse_input(contents: &str) -> Vec<(&str, &str)> {
    contents
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .collect()
}

fn part1(contents: &str) -> i32 {
    let system = parse_input(contents);
    let mut paths: Vec<Vec<&str>> = vec![];
    for (src, dst) in system.iter().filter(|(src, _)| *src == "start") {
        let mut path = vec![*src, *dst];
        println!("{}-{}", src, dst);
        paths.push(path);
    }

    paths
        .iter()
        .filter(|p| p.iter().any(|c| *c == c.to_lowercase()))
        .count() as i32
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
    fn test_part_1_example_1() {
        let contents = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(part1(contents), 6);
    }

    #[test]
    fn test_part_1_example_2() {
        let contents = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(part1(contents), 7);
    }

    /*
        #[test]
        fn test_part_1_example_3() {
            let contents = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";
            assert_eq!(part1(contents), 226);
        }
        */
}
