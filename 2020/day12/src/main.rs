enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Instruction {
    direction: Direction,
    value: i32,
}

struct Ship {
    east: i32,
    north: i32,
    direction: Direction,
}

impl Ship {
    fn new() -> Self {
        Ship {
            east: 0,
            north: 0,
            direction: Direction::East,
        }
    }

    fn turn_ship(&mut self, instruction: Instruction) {
        // Normalize turn to only use right turns
        let instruction = match instruction.direction {
            Direction::Left => match instruction.value {
                90 => Instruction {
                    direction: Direction::Right,
                    value: 270,
                },
                180 => Instruction {
                    direction: Direction::Right,
                    value: 180,
                },
                270 => Instruction {
                    direction: Direction::Right,
                    value: 90,
                },
                _ => panic!("Invalid amount for ship to turn"),
            },
            Direction::Right => instruction,
            _ => panic!("Invalid direction for ship to turn"),
        };

        self.direction = match self.direction {
            Direction::North => match instruction.value {
                90 => Direction::East,
                180 => Direction::South,
                270 => Direction::West,
                _ => panic!("Invalid amount for ship to turn"),
            },
            Direction::South => match instruction.value {
                90 => Direction::West,
                180 => Direction::North,
                270 => Direction::East,
                _ => panic!("Invalid amount for ship to turn"),
            },
            Direction::East => match instruction.value {
                90 => Direction::South,
                180 => Direction::West,
                270 => Direction::North,
                _ => panic!("Invalid amount for ship to turn"),
            },
            Direction::West => match instruction.value {
                90 => Direction::North,
                180 => Direction::East,
                270 => Direction::South,
                _ => panic!("Invalid amount for ship to turn"),
            },
            _ => panic!("Invalid direction for ship to face"),
        }
    }

    fn move_ship(&mut self, instruction: Instruction) {
        match instruction.direction {
            Direction::North => self.north += instruction.value,
            Direction::South => self.north -= instruction.value,
            Direction::East => self.east += instruction.value,
            Direction::West => self.east -= instruction.value,
            Direction::Left | Direction::Right => self.turn_ship(instruction),
            Direction::Forward => match self.direction {
                Direction::North => self.north += instruction.value,
                Direction::South => self.north -= instruction.value,
                Direction::East => self.east += instruction.value,
                Direction::West => self.east -= instruction.value,
                _ => panic!("Invalid direction for ship to face"),
            },
        };
    }

    fn manhattan_distance(&self) -> i32 {
        self.east.abs() + self.north.abs()
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let action = line.chars().nth(0).unwrap();
            let value = line
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            let direction = match action {
                'N' => Direction::North,
                'S' => Direction::South,
                'E' => Direction::East,
                'W' => Direction::West,
                'L' => Direction::Left,
                'R' => Direction::Right,
                'F' => Direction::Forward,
                _ => panic!("Unknown action: {}", action),
            };

            Instruction { direction, value }
        })
        .collect::<Vec<Instruction>>()
}

fn part1(input: Vec<Instruction>) -> i32 {
    let mut ship = Ship::new();

    for instruction in input {
        ship.move_ship(instruction);
    }

    ship.manhattan_distance()
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(parse_input(&contents));
    println!("part1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part1};

    #[test]
    fn test_part1_example() {
        let input = r#"F10
N3
F7
R90
F11"#;
        assert_eq!(part1(parse_input(input)), 25);
    }
}
