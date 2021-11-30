use std::collections::HashMap;

fn contains_bag(map: &HashMap<&str, Vec<(&str, usize)>>, bag: &str) -> bool {
    match map.get(bag) {
        Some(bag_list) => {
            if bag_list.is_empty() {
                return false;
            }
            if bag_list
                .iter()
                .find(|(desc, _)| *desc == "shiny gold")
                .is_some()
            {
                return true;
            }
            for nested_bag in bag_list {
                if contains_bag(map, nested_bag.0) {
                    return true;
                }
            }
            return false;
        }
        None => panic!("Invalid bag description"),
    }
}

fn count_bags(map: &HashMap<&str, Vec<(&str, usize)>>, bag: &str) -> usize {
    map.get(bag)
        .unwrap()
        .iter()
        .map(|(nested_bag, cnt)| cnt + (cnt * count_bags(map, nested_bag)))
        .sum()
}

fn create_map(input: &str) -> HashMap<&str, Vec<(&str, usize)>> {
    let mut map: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();
    for l in input.lines() {
        let split = l.splitn(2, " bags contain ").collect::<Vec<&str>>();
        let (description, content) = (split[0], split[1]);
        match content {
            "no other bags." => {
                map.entry(description).or_insert(vec![]);
            }
            _ => {
                let content = content
                    .trim_end_matches('.')
                    .split(", ")
                    .map(|x| {
                        let n = x.chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
                        let desc = *x
                            .get(2..)
                            .unwrap()
                            .rsplitn(2, ' ')
                            .collect::<Vec<&str>>()
                            .get(1)
                            .unwrap();
                        (desc, n)
                    })
                    .collect();
                map.entry(description).or_insert(content);
            }
        }
    }
    map
}

fn part1(input: &str) -> usize {
    let map = create_map(input);
    map.iter()
        .filter(|(k, _)| contains_bag(&map, k))
        .collect::<Vec<(&&str, &Vec<(&str, usize)>)>>()
        .len()
}

fn part2(input: &str) -> usize {
    let map = create_map(input);
    count_bags(&map, "shiny gold")
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(&contents);
    println!("part1: {}", part1);

    let part2 = part2(&contents);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let r = part1(
            r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#,
        );
        assert_eq!(r, 4);
    }

    #[test]
    fn test_part2_example_1() {
        let r = part2(
            r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#,
        );
        assert_eq!(r, 32);
    }

    #[test]
    fn test_part2_example_2() {
        let r = part2(
            r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#,
        );
        assert_eq!(r, 126);
    }
}
