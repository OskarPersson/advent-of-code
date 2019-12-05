use std::fs;

fn computer(contents: String, noun: i32, verb: i32) -> i32 {
    let mut ints: Vec<i32> = contents
        .split_terminator(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    ints[1] = noun;
    ints[2] = verb;

    let mut pos: usize = 0;
    let mut code: i32 = ints[pos];

    while code != 99 {
        let mod_pos = ints[pos + 3] as usize;
        let first_pos = ints[pos + 1] as usize;
        let second_pos = ints[pos + 2] as usize;
        match code {
            1 => {
                ints[mod_pos] = ints[first_pos] + ints[second_pos];
                pos += 4;
            }
            2 => {
                ints[mod_pos] = ints[first_pos] * ints[second_pos];
                pos += 4;
            }
            _ => {}
        }
        code = ints[pos];
    }
    return ints[0];
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let part1 = computer(contents.clone(), 12, 2);
    println!("part1: {}", part1);

    let mut output: i32 = 0;
    let mut noun = -1;
    let mut verb = -1;
    while noun < 100 && output != 19690720 {
        verb = -1;
        noun += 1;
        while verb < 100 && output != 19690720 {
            verb += 1;
            output = computer(contents.clone(), noun, verb);
        }
    }
    
    println!("noun: {}, verb: {}", noun, verb);
    let part2 = 100 * noun + verb;
    println!("part2: {}", part2);
}
