use std::cmp::Ordering;

use regex::Regex;

fn parse_input(contents: &str) -> (i32, i32, i32, i32) {
    let re = Regex::new(r"target area: x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let caps = re.captures(contents).unwrap();
    (
        caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
    )
}

struct Probe {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
    trajectory: Vec<(i32, i32)>,
}

impl Probe {
    fn new(vel_x: i32, vel_y: i32) -> Self {
        Probe {
            pos_x: 0,
            pos_y: 0,
            vel_x,
            vel_y,
            trajectory: vec![(0, 0)],
        }
    }

    fn is_past_target_area(&self, target_max_x: i32, target_min_y: i32) -> bool {
        self.pos_x > target_max_x || self.pos_y < target_min_y
    }

    fn is_in_target_area(
        &self,
        target_min_x: i32,
        target_max_x: i32,
        target_min_y: i32,
        target_max_y: i32,
    ) -> bool {
        target_min_x <= self.pos_x
            && self.pos_x <= target_max_x
            && target_min_y <= self.pos_y
            && self.pos_y <= target_max_y
    }

    fn get_highest_pos(&self) -> i32 {
        *self.trajectory.iter().map(|(_, y)| y).max().unwrap()
    }

    fn step(&mut self) {
        // * The probe's x position increases by its x velocity.
        // * The probe's y position increases by its y velocity.
        // * Due to drag, the probe's x velocity changes by 1 toward the value 0; that is,
        //   it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0,
        //   or does not change if it is already 0.
        // * Due to gravity, the probe's y velocity decreases by 1.

        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;

        match self.vel_x.cmp(&0) {
            Ordering::Greater => self.vel_x -= 1,
            Ordering::Less => self.vel_x += 1,
            Ordering::Equal => {}
        }
        self.vel_y -= 1;

        self.trajectory.push((self.pos_x, self.pos_y));
    }
}

fn part1(contents: &str) -> i32 {
    let (min_x, max_x, min_y, max_y) = parse_input(contents);
    let mut highest_positions = vec![];

    for vel_x in 0..1000 {
        for vel_y in 0..1000 {
            let mut probe = Probe::new(vel_x, vel_y);
            while !probe.is_past_target_area(max_x, min_y) {
                probe.step();
                if probe.is_in_target_area(min_x, max_x, min_y, max_y) {
                    highest_positions.push(probe.get_highest_pos());
                    break;
                }
            }
        }
    }
    return *highest_positions.iter().max().unwrap();
}

fn part2(contents: &str) -> i32 {
    let (min_x, max_x, min_y, max_y) = parse_input(contents);
    let mut successful_velocities = 0;

    for vel_x in 0..1000 {
        for vel_y in -1000..1000 {
            let mut probe = Probe::new(vel_x, vel_y);
            while !probe.is_past_target_area(max_x, min_y) {
                probe.step();
                if probe.is_in_target_area(min_x, max_x, min_y, max_y) {
                    successful_velocities += 1;
                    break;
                }
            }
        }
    }
    successful_velocities
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
    fn test_parse_input() {
        assert_eq!(
            parse_input("target area: x=20..30, y=-10..-5"),
            (20, 30, -10, -5)
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1("target area: x=20..30, y=-10..-5"), 45);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2("target area: x=20..30, y=-10..-5"), 112);
    }
}
