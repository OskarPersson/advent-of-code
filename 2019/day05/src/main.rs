use std::fs;

struct Computer {
    data: Vec<i32>,
    code: i32,
    pos: usize,
}

impl Computer {
    fn new(data: String) -> Computer {
        let ints: Vec<i32> = data
            .split_terminator(',')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        let start_code = ints[0];
        Computer {
            data: ints,
            code: start_code,
            pos: 0,
        }
    }

    fn get_opcode(&self) -> i32 {
        let code_str = self.code.to_string();
        if code_str.len() < 3 {
            return self.code;
        }
        return code_str[code_str.len() - 2..].parse::<i32>().unwrap();
    }

    fn get_param_modes(&self) -> Vec<i32> {
        let code_str = self.code.to_string();
        if code_str.len() < 3 {
            return vec![];
        }
        return code_str[..code_str.len() - 2]
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .rev()
            .collect();
    }

    fn get_params(&self, n: usize) -> Vec<i32> {
        let param_modes = self.get_param_modes();
        let param_modes_len = param_modes.len();
        let mut vec = Vec::new();

        for i in 0..n {
            let param_mode = if i < param_modes_len {
                param_modes[i]
            } else {
                0
            };
            let param = match param_mode {
                0 => {
                    let data_pos = self.data[self.pos + 1 + i] as usize;
                    self.data[data_pos]
                } // position mode
                1 => self.data[self.pos + 1 + i], // immediate mdoe
                _ => unreachable!(),
            };
            vec.push(param);
        }

        return vec;
    }

    fn compute(&mut self, input: i32) -> i32 {
        let mut opcode = self.get_opcode();
        let mut output = 0;

        while opcode != 99 {
            match opcode {
                1 => {
                    let params = self.get_params(2);
                    let mod_pos = self.data[self.pos + 3] as usize;
                    self.data[mod_pos] = params[0] + params[1];
                    self.pos += 4;
                }
                2 => {
                    let params = self.get_params(2);
                    let mod_pos = self.data[self.pos + 3] as usize;
                    self.data[mod_pos] = params[0] * params[1];
                    self.pos += 4;
                }
                3 => {
                    let mod_pos = self.data[self.pos + 1] as usize;
                    self.data[mod_pos] = input;
                    self.pos += 2;
                }
                4 => {
                    let read_pos = self.data[self.pos + 1] as usize;
                    output = self.data[read_pos];
                    self.pos += 2;
                }
                5 => {
                    let params = self.get_params(2);
                    if params[0] != 0 {
                        self.pos = params[1] as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                6 => {
                    let params = self.get_params(2);
                    if params[0] == 0 {
                        self.pos = params[1] as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                7 => {
                    let params = self.get_params(2);
                    let mod_pos = self.data[self.pos + 3] as usize;
                    self.data[mod_pos] = if params[0] < params[1] { 1 } else { 0 };
                    self.pos += 4;
                }
                8 => {
                    let params = self.get_params(2);
                    let mod_pos = self.data[self.pos + 3] as usize;
                    self.data[mod_pos] = if params[0] == params[1] { 1 } else { 0 };
                    self.pos += 4;
                }
                _ => unreachable!(),
            }
            self.code = self.data[self.pos];
            opcode = self.get_opcode();
        }
        return output;
    }
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut computer = Computer::new(contents.clone());
    let part1 = computer.compute(1);

    computer = Computer::new(contents);
    let part2 = computer.compute(5);

    println!("part1: {}, part2: {}", part1, part2);
}
