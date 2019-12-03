use std::fs;

fn calculate_fuel(fuel: f32) -> f32 {
    return match fuel {
       x if x <= 0.0 => 0.0,
        x => x + calculate_fuel((x/3.0).floor() - 2.0)
    }
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let masses = contents
        .lines()
        .map(|line| ((line.parse::<f32>().unwrap()/3.0).floor() - 2.0));
    println!("part1 {}", masses.clone().sum::<f32>());

    let fuel_reqs = masses.map(|mass| (calculate_fuel(mass)));
    println!("part2 {}", fuel_reqs.sum::<f32>());
}
