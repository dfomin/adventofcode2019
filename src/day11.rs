use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

#[derive(Debug)]
enum IntCodeState {
    Created,
    WaitInput,
    Halted,
}

struct IntCode {
    memory: Vec<i64>,
    index: usize,
    input: VecDeque<i64>,
    output: Vec<i64>,
    state: IntCodeState,
    relative_base: i64,
}

impl IntCode {
    fn new(memory: Vec<i64>, input: VecDeque<i64>) -> Self {
        Self {
            memory,
            index: 0,
            input,
            output: vec![],
            state: IntCodeState::Created,
            relative_base: 0,
        }
    }

    fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
        if matches!(self.state, IntCodeState::WaitInput) {
            self.execute()
        }
    }

    fn parameter(&self, mode_index: usize, mut modes: i64) -> i64 {
        for _ in 0..mode_index - 1 {
            modes /= 10;
        }
        let mode = modes % 10;
        let index = self.index + mode_index;
        match mode {
            0 => self.memory[index],
            1 => index as i64,
            2 => self.memory[index] + self.relative_base,
            _ => panic!(""),
        }
    }

    fn memory_value(&mut self, index: usize) -> &mut i64 {
        while index >= self.memory.len() {
            self.memory.push(0);
        }
        &mut self.memory[index]
    }

    fn execute(&mut self) {
        loop {
            let op = self.memory[self.index] % 100;
            let modes = self.memory[self.index] / 100;

            let op_count = match op {
                1 | 2 | 7 | 8 => 4,
                3 | 4 => 2,
                5 | 6 => 3,
                9 => 2,
                99 => 1,
                _ => panic!("Unexpected command: {}", self.memory[self.index]),
            };

            let ops = (1..op_count)
                .map(|i| self.parameter(i, modes))
                .collect::<Vec<_>>();
            match op {
                1 => {
                    *self.memory_value(ops[2] as usize) =
                        *self.memory_value(ops[0] as usize) + *self.memory_value(ops[1] as usize);
                    self.index += 4;
                }
                2 => {
                    *self.memory_value(ops[2] as usize) =
                        *self.memory_value(ops[0] as usize) * *self.memory_value(ops[1] as usize);
                    self.index += 4;
                }
                3 => {
                    if let Some(next_input) = self.input.pop_front() {
                        *self.memory_value(ops[0] as usize) = next_input;
                        self.index += 2;
                    } else {
                        self.state = IntCodeState::WaitInput;
                        break;
                    }
                }
                4 => {
                    let value = *self.memory_value(ops[0] as usize);
                    self.output.push(value);
                    self.index += 2;
                }
                5 => {
                    if *self.memory_value(ops[0] as usize) != 0 {
                        self.index = *self.memory_value(ops[1] as usize) as usize;
                    } else {
                        self.index += 3;
                    }
                }
                6 => {
                    if *self.memory_value(ops[0] as usize) == 0 {
                        self.index = *self.memory_value(ops[1] as usize) as usize;
                    } else {
                        self.index += 3;
                    }
                }
                7 => {
                    if *self.memory_value(ops[0] as usize) < *self.memory_value(ops[1] as usize) {
                        *self.memory_value(ops[2] as usize) = 1;
                    } else {
                        *self.memory_value(ops[2] as usize) = 0;
                    }
                    self.index += 4;
                }
                8 => {
                    if *self.memory_value(ops[0] as usize) == *self.memory_value(ops[1] as usize) {
                        *self.memory_value(ops[2] as usize) = 1;
                    } else {
                        *self.memory_value(ops[2] as usize) = 0;
                    }
                    self.index += 4;
                }
                9 => {
                    self.relative_base += *self.memory_value(ops[0] as usize);
                    self.index += 2;
                }
                99 => {
                    self.state = IntCodeState::Halted;
                    break;
                }
                _ => panic!("Unexpected command: {}", self.memory[self.index]),
            }
        }
    }
}

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>()
}

fn solve(input: &str, start: i64) -> (HashMap<(i64, i64), i64>, (i64, i64, i64, i64)) {
    let values = parse(input);
    let mut int_code = IntCode::new(values, VecDeque::new());
    let mut dir = (0, 1);
    let mut pos = (0, 0);
    let mut points = HashMap::new();
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    points.insert(pos, start);
    while !matches!(int_code.state, IntCodeState::Halted) {
        let color = if let Some(&color) = points.get(&pos) {
            color
        } else {
            0
        };

        int_code.add_input(color);
        int_code.execute();

        let new_color = int_code.output[int_code.output.len() - 2];
        let turn = int_code.output[int_code.output.len() - 1];

        points.insert(pos, new_color);
        if turn == 0 {
            dir = (-dir.1, dir.0);
        } else {
            dir = (dir.1, -dir.0);
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);

        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        min_y = min_y.min(pos.1);
        max_y = max_y.max(pos.1);
    }
    (points, (min_x, min_y, max_x, max_y))
}

pub fn part1(input: &str) -> i64 {
    solve(input, 0).0.len() as i64
}

pub fn part2(input: &str) -> String {
    let (map, (min_x, min_y, max_x, max_y)) = solve(input, 1);
    let mut chars = vec![vec![b' '; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for (key, value) in map {
        if value == 1 {
            chars[(max_y - key.1) as usize][(key.0 - min_x) as usize] = b'#';
        }
    }
    chars
        .iter()
        .map(|row| std::str::from_utf8(row).unwrap())
        .join("\n")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day11_part1() {}

    #[test]
    fn test_day11_part2() {}
}
