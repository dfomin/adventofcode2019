use std::collections::VecDeque;

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

fn solve(input: &str, input_parameter: i64) -> String {
    let values = parse(input);
    let mut int_code = IntCode::new(values, VecDeque::from([input_parameter]));
    int_code.execute();
    int_code.output.iter().map(|x| x.to_string()).join(",")
}

pub fn part1(input: &str) -> String {
    solve(input, 1)
}

pub fn part2(input: &str) -> String {
    solve(input, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_part1() {
        assert_eq!(
            part1("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"),
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
        );

        assert_eq!(
            part1("1102,34915192,34915192,7,4,7,99,0"),
            "1219070632396864"
        );

        assert_eq!(part1("104,1125899906842624,99"), "1125899906842624");
    }

    #[test]
    fn test_day9_part2() {}
}