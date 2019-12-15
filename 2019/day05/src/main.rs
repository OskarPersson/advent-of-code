use aoc2019::intcode::Computer;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut computer = Computer::new(contents.clone());
    let part1 = computer.compute(&[1]);

    computer = Computer::new(contents);
    let part2 = computer.compute(&[5]);

    println!("part1: {}, part2: {}", part1, part2);
}
