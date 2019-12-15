use aoc2019::intcode::Computer;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut c = Computer::new(contents.clone());
    c.set(1, 12);
    c.set(2, 2);
    c.compute(&[]);
    let part1 = c.get(0);
    println!("part1: {}", part1);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut output: i32 = 0;
    let mut noun = -1;
    let mut verb = -1;
    while noun < 100 && output != 19690720 {
        verb = -1;
        noun += 1;
        while verb < 100 && output != 19690720 {
            verb += 1;
            let mut c = Computer::new(contents.clone());
            c.set(1, noun);
            c.set(2, verb);
            c.compute(&[]);
            output = c.get(0);
        }
    }

    let part2 = 100 * noun + verb;
    println!("part2: {}", part2);
}
