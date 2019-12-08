fn part1(password: i32) -> bool {
    let pass_str = password.to_string();
    if pass_str.chars().count() != 6 {
        return false;
    }

    let mut last_char: Option<char> = None;
    let mut has_adjacent_duplicate = false;

    for c in pass_str.chars() {
        if let Some(lc) = last_char {
            if lc == c {
                has_adjacent_duplicate = true;
            }
            // check if decreased
            else if lc.to_digit(10) > c.to_digit(10) {
                return false;
            }
        }
        last_char = Some(c);
    }

    return has_adjacent_duplicate;
}

fn part2(password: i32) -> bool {
    let pass_str = password.to_string();

    let mut last_char: Option<char> = None;
    let mut has_adjacent_duplicate = false;
    let mut adjacent_duplicate_count = 0;

    for c in pass_str.chars() {
        if let Some(lc) = last_char {
            if lc == c {
                adjacent_duplicate_count += 1;
            } else {
                if adjacent_duplicate_count == 1 {
                    has_adjacent_duplicate = true;
                }
                adjacent_duplicate_count = 0
            }
        }
        last_char = Some(c);
    }

    return has_adjacent_duplicate || adjacent_duplicate_count == 1;
}

fn main() {
    let input = 240298..784956;

    let mut part1_count = 0;
    let mut part2_count = 0;

    for i in input {
        if part1(i) {
            part1_count += 1;

            if part2(i) {
                part2_count += 1;
            }
        }
    }
    println!("part1: {}", part1_count);
    println!("part2: {}", part2_count);
}
