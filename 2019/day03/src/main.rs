use std::{cmp, fs};

type Intersection = Vec<(i32, i32, i32)>;

fn find_intersections(path1: Vec<(i32, i32, i32)>, path2: Vec<(i32, i32, i32)>) -> Intersection {
    let mut matches = vec![];
    for i in 0..path1.len() - 1 {
        let path1_xmin = cmp::min(path1[i].0, path1[i + 1].0);
        let path1_xmax = cmp::max(path1[i].0, path1[i + 1].0);

        let path1_ymin = cmp::min(path1[i].1, path1[i + 1].1);
        let path1_ymax = cmp::max(path1[i].1, path1[i + 1].1);

        let xrange = path1_xmin..path1_xmax;
        let yrange = path1_ymin..path1_ymax;
        for j in 0..path2.len() - 1 {
            let path2_xmin = cmp::min(path2[j].0, path2[j + 1].0);
            let path2_xmax = cmp::max(path2[j].0, path2[j + 1].0);

            let path2_ymin = cmp::min(path2[j].1, path2[j + 1].1);
            let path2_ymax = cmp::max(path2[j].1, path2[j + 1].1);

            if xrange.contains(&path2[j].0) {
                let yrange = path2_ymin..path2_ymax;
                if yrange.contains(&path1[i].1) {
                    matches.push((
                        path2[j].0,
                        path1[i].1,
                        path1[i].2
                            + (path2[j].0 - path1[i].0).abs()
                            + path2[j].2
                            + (path1[i].0 - path2[j].0).abs(),
                    ));
                }
            }
            if yrange.contains(&path2[j].1) {
                let xrange = path2_xmin..path2_xmax;

                if xrange.contains(&path1[i].0) {
                    matches.push((
                        path1[i].0,
                        path2[j].1,
                        path1[i].2
                            + (path2[j].1 - path1[i].1).abs()
                            + path2[j].2
                            + (path1[i].0 - path2[j].0).abs(),
                    ));
                }
            }
        }
    }
    return matches;
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let wires: Vec<&str> = contents.lines().collect();
    let wire1_path = wires[0].split_terminator(',');
    let wire2_path = wires[1].split_terminator(',');

    let mut wire1_coord: Vec<(i32, i32, i32)> = vec![(0, 0, 0)]; // x, y
    let mut wire2_coord: Vec<(i32, i32, i32)> = vec![(0, 0, 0)];

    for instruction in wire1_path {
        let direction = instruction.chars().next().unwrap();
        let steps = &instruction[1..].parse::<i32>().unwrap();
        let last_coord = wire1_coord.last().unwrap();
        let total_steps = last_coord.2 + steps;

        let new_coord = match direction {
            'U' => (last_coord.0, last_coord.1 - steps, total_steps),
            'R' => (last_coord.0 + steps, last_coord.1, total_steps),
            'D' => (last_coord.0, last_coord.1 + steps, total_steps),
            'L' => (last_coord.0 - steps, last_coord.1, total_steps),
            _ => unreachable!(),
        };
        wire1_coord.push(new_coord);
    }

    for instruction in wire2_path {
        let direction = instruction.chars().next().unwrap();
        let steps = &instruction[1..].parse::<i32>().unwrap();
        let last_coord = wire2_coord.last().unwrap();
        let total_steps = last_coord.2 + steps;

        let new_coord = match direction {
            'U' => (last_coord.0, last_coord.1 - steps, total_steps),
            'R' => (last_coord.0 + steps, last_coord.1, total_steps),
            'D' => (last_coord.0, last_coord.1 + steps, total_steps),
            'L' => (last_coord.0 - steps, last_coord.1, total_steps),
            _ => unreachable!(),
        };
        wire2_coord.push(new_coord);
    }

    let intersections = find_intersections(wire1_coord, wire2_coord);
    let mut shortest_dist: i32 = 0;

    for intersection in intersections.clone() {
        let dist = intersection.0.abs() + intersection.1.abs();
        if shortest_dist == 0 || dist < shortest_dist {
            shortest_dist = dist;
        }
    }

    println!("part1: {}", shortest_dist);

    let mut shortest_path: i32 = 0;

    for intersection in intersections {
        let path = intersection.2;
        if shortest_path == 0 || path < shortest_path {
            shortest_path = path;
        }
    }

    println!("part2: {}", shortest_path);
}
