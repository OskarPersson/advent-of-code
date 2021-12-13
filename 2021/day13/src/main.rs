#[derive(PartialEq)]
struct Dot {
    pub x: i32,
    pub y: i32,
}

enum FoldDir {
    Left,
    Up,
}

struct Fold {
    pub dir: FoldDir,
    pub idx: i32,
}

fn parse_input(contents: &str) -> (Vec<Dot>, Vec<Fold>) {
    let (dots, folds) = contents.split_once("\n\n").unwrap();

    let dots: Vec<Dot> = dots
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Dot {
                x: x.parse::<i32>().unwrap(),
                y: y.parse::<i32>().unwrap(),
            }
        })
        .collect();

    let folds: Vec<Fold> = folds
        .lines()
        .map(|l| {
            let (_, instr) = l.rsplit_once(' ').unwrap();
            let (pos, val) = instr.split_once('=').unwrap();
            Fold {
                dir: if pos.chars().next().unwrap() == 'x' {
                    FoldDir::Left
                } else {
                    FoldDir::Up
                },
                idx: val.parse::<i32>().unwrap(),
            }
        })
        .collect();

    (dots, folds)
}

fn fold(dots: &mut Vec<Dot>, instr: &Fold) {
    match instr.dir {
        FoldDir::Left => {
            for dot in dots.iter_mut().filter(|p| p.x > instr.idx) {
                dot.x = instr.idx - (dot.x - instr.idx);
            }
        }
        FoldDir::Up => {
            for dot in dots.iter_mut().filter(|p| p.y > instr.idx) {
                dot.y = instr.idx - (dot.y - instr.idx);
            }
        }
    }
}

fn part1(contents: &str) -> i32 {
    let (mut dots, folds) = parse_input(contents);
    fold(&mut dots, folds.get(0).unwrap());

    dots.sort_unstable_by_key(|dot| (dot.x, dot.y));
    dots.dedup();
    dots.len() as i32
}

fn part2(contents: &str) {
    let (mut dots, folds) = parse_input(contents);

    for f in folds {
        fold(&mut dots, &f);
    }
    dots.sort_unstable_by_key(|dot| (dot.x, dot.y));
    dots.dedup();

    let max_x = dots.iter().max_by_key(|d| d.x).unwrap().x;
    let max_y = dots.iter().max_by_key(|d| d.y).unwrap().y;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.iter().find(|d| d.x == x && d.y == y).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(contents);
    println!("part1: {}", part1);

    println!("part2:");
    part2(contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let contents = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!(part1(contents), 17);
    }
}
