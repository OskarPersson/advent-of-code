type System<'a> = Vec<(&'a str, &'a str)>;

fn parse_input(contents: &str) -> System {
    contents
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .collect()
}

fn visit_adjacent_nodes<'a>(
    system: &'a System,
    path: &mut Vec<&'a str>,
    paths: &mut Vec<Vec<&'a str>>,
    duplicate_small_node: bool,
) {
    let node = *path.clone().last().unwrap();
    for (_, adj_dst) in system
        .iter()
        .filter_map(|(src, dst)| {
            if *src == node {
                Some((src, dst))
            } else if *dst == node {
                Some((dst, src))
            } else {
                None
            }
        })
        .filter(|(_, dst)| **dst != "start")
    {
        let mut dup = duplicate_small_node;
        if **adj_dst == adj_dst.to_lowercase() && path.contains(adj_dst) {
            if dup {
                dup = false;
            } else {
                continue;
            }
        }

        let mut new_path = path.clone();
        new_path.push(adj_dst);

        if *adj_dst == "end" {
            paths.push(new_path);
        } else {
            visit_adjacent_nodes(system, &mut new_path, paths, dup)
        }
    }
}

fn part1(contents: &str) -> i32 {
    let system = parse_input(contents);
    let mut paths: Vec<Vec<&str>> = vec![];
    for (src, dst) in system.iter().filter_map(|(src, dst)| {
        if *src == "start" {
            Some((src, dst))
        } else if *dst == "start" {
            Some((dst, src))
        } else {
            None
        }
    }) {
        let mut path = vec![*src, *dst];
        visit_adjacent_nodes(&system, &mut path, &mut paths, false);
    }
    paths.len() as i32
}

fn part2(contents: &str) -> i32 {
    let system = parse_input(contents);
    let mut paths: Vec<Vec<&str>> = vec![];
    for (src, dst) in system.iter().filter_map(|(src, dst)| {
        if *src == "start" {
            Some((src, dst))
        } else if *dst == "start" {
            Some((dst, src))
        } else {
            None
        }
    }) {
        let mut path = vec![*src, *dst];
        visit_adjacent_nodes(&system, &mut path, &mut paths, true);
    }
    paths.len() as i32
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(contents);
    println!("part1: {}", part1);

    let part2 = part2(contents);
    println!("part2: {}", part2);
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
        assert_eq!(part1(contents), 10);
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
        assert_eq!(part1(contents), 19);
    }

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

    #[test]
    fn test_part_2_example_1() {
        let contents = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(part2(contents), 36);
    }

    #[test]
    fn test_part_2_example_2() {
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
        assert_eq!(part2(contents), 103);
    }

    #[test]
    fn test_part_2_example_3() {
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
        assert_eq!(part2(contents), 3509);
    }
}
