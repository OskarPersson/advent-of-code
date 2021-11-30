
fn parse_line(line: &str) -> (&str, i32) {
    let full_cmd: Vec<&str> = line.split(' ').collect();
    let num = full_cmd.get(1).unwrap().parse::<i32>().unwrap();
    let cmd = *full_cmd.get(0).unwrap();
    (cmd, num)
}

fn step(line: &str, mut acc: i32, mut pos: i32) -> (i32, i32) {
    let (cmd, num) = parse_line(line);
    match cmd {
        "acc" => {
            acc += num;
            pos += 1;
        }
        "jmp" => {
            pos += num;
        }
        "nop" => {
            pos += 1;
        }
        _ => unreachable!("Unknown command {}", cmd),
    };
    (pos, acc)
}

fn run(commands: &Vec<String>) -> Result<i32, i32> {
    let mut visited: Vec<i32> = vec![];
    let mut pos = 0;
    let mut acc = 0;

    while !visited.contains(&pos) && pos < commands.len() as i32 {
        visited.push(pos);
        let res = step(commands.get(pos as usize).unwrap(), acc, pos);
        pos = res.0;
        acc = res.1;
    }

    if pos == commands.len() as i32 {
        return Ok(acc);
    }
    Err(acc)
}

fn part1(input: &str) -> i32 {
    let commands: Vec<String> = input.lines().map(|s| String::from(s)).collect();
    run(&commands).unwrap_err()
}

fn part2(input: &str) -> i32 {
    let commands: Vec<String> = input.lines().map(|s| String::from(s)).collect();
    let mut pos = 0;
    let mut res = run(&commands);

    while res.is_err() {
        let mut commands: Vec<String> = input.lines().map(|s| String::from(s)).collect();
        let (cmd, num) = parse_line(commands.get(pos).unwrap());

        commands[pos] = match cmd {
            "nop" => format!("jmp {}", num),
            "jmp" => format!("nop {}", num),
            _ => format!("{} {}", cmd, num),
        };

        res = run(&commands);

        pos += 1;
    }

    res.unwrap()
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
            r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#,
        );
        assert_eq!(r, 5);
    }

    #[test]
    fn test_part2() {
        let r = part2(
            r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#,
        );
        assert_eq!(r, 8);
    }
}
