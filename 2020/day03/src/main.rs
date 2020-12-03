use std::fs;

fn tree_counter(contents: &str, right: usize, down: usize) -> usize {
    let line_len = contents.lines().nth(0).unwrap().len();
    let mut pos = 0;
    let mut skips_left = 0;
    return contents
        .lines()
        .filter_map(|line| {
            if skips_left > 0 {
                skips_left = skips_left - 1;
                return None;
            }
            skips_left = down - 1;
            let char_at_pos = line.chars().nth(pos).unwrap();

            pos = pos + right;
            pos = pos % line_len;

            match char_at_pos {
                '#' => Some(()),
                _ => None,
            }
        })
        .collect::<Vec<()>>()
        .len();
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let part1 = tree_counter(&contents, 3, 1);
    println!("part1: {}", part1);

    let part2 = tree_counter(&contents, 1, 1)
        * tree_counter(&contents, 3, 1)
        * tree_counter(&contents, 5, 1)
        * tree_counter(&contents, 7, 1)
        * tree_counter(&contents, 1, 2);
    println!("part2: {}", part2);
}
