struct Board {
    rows: Vec<Vec<(i32, bool)>>,
    cols: Vec<Vec<(i32, bool)>>,
    won: bool,
}

impl Board {
    pub fn new(numbers: Vec<i32>) -> Self {
        let rows: Vec<Vec<(i32, bool)>> = numbers
            .chunks(5)
            .map(|row| row.into_iter().map(|n| (*n, false)).collect())
            .collect();

        let mut cols: Vec<Vec<(i32, bool)>> = vec![];

        for idx in 0..4 {
            let mut col = vec![];
            for row in &rows {
                col.push(row[idx]);
            }
            cols.push(col);
        }

        Board {
            rows,
            cols,
            won: false,
        }
    }

    pub fn mark(&mut self, number: i32) -> () {
        for row in self.rows.iter_mut() {
            for n in row.iter_mut() {
                *n = (n.0, n.1 || n.0 == number);
            }
        }

        for col in self.cols.iter_mut() {
            for n in col.iter_mut() {
                *n = (n.0, n.1 || n.0 == number);
            }
        }
    }

    pub fn check(&mut self) -> Option<i32> {
        for row in &self.rows {
            if row
                .iter()
                .filter(|(_, m)| !m)
                .collect::<Vec<&(i32, bool)>>()
                .is_empty()
            {
                self.won = true;
                return Some(self.sum_unmarked());
            }
        }

        for col in &self.cols {
            if col
                .into_iter()
                .filter(|(_, m)| !m)
                .collect::<Vec<&(i32, bool)>>()
                .is_empty()
            {
                self.won = true;
                return Some(self.sum_unmarked());
            }
        }

        return None;
    }

    fn sum_unmarked(&self) -> i32 {
        self.rows
            .clone()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .filter_map(|(n, marked)| {
                        if !marked {
                            return Some(n);
                        }
                        return None;
                    })
                    .sum::<i32>()
            })
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<Board>) {
    let mut entries = input.split("\n\n");

    let numbers: Vec<i32> = entries
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let boards = entries
        .map(|e| {
            Board::new(
                e.replace("  ", " ")
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            )
        })
        .collect();

    (numbers, boards)
}

fn part1(numbers: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for n in numbers {
        for b in &mut boards {
            b.mark(n);

            if let Some(res) = b.check() {
                return res * n;
            }
        }
    }

    return 0;
}

fn part2(numbers: Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for n in numbers {
        let unfinished: Vec<&mut Board> = boards.iter_mut().filter(|b| !b.won).collect();
        let unfinished_len = unfinished.len();
        for b in unfinished {
            b.mark(n);

            if let Some(res) = b.check() {
                if unfinished_len == 1 {
                    return res * n;
                }
            }
        }
    }

    return 0;
}

fn main() {
    let contents = include_str!("../input.txt");

    let (numbers, boards) = parse_input(contents);
    let part1 = part1(numbers, boards);
    println!("part1: {}", part1);

    let (numbers, boards) = parse_input(contents);
    let part2 = part2(numbers, boards);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let contents = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let (numbers, boards) = parse_input(contents);
        assert_eq!(part1(numbers, boards), 4512);
    }

    #[test]
    fn test_part_2() {
        let contents = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let (numbers, boards) = parse_input(contents);
        assert_eq!(part2(numbers, boards), 1924);
    }
}
