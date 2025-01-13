use std::collections::VecDeque;

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

pub fn part1(input: &str) -> i64 {
    let values = parse(input);
    let mut int_code = IntCode::new(values, VecDeque::new());
    int_code.execute();
    int_code
        .output
        .chunks(3)
        .filter(|&triple| triple.len() == 3 && triple[2] == 2)
        .count() as i64
}

pub fn part2(input: &str) -> i64 {
    let mut values = parse(input);
    values[0] = 2;
    let mut int_code = IntCode::new(values, VecDeque::new());
    int_code.execute();
    let (max_x, max_y) = int_code.output.chunks(3).fold((0, 0), |acc, chunk| {
        (acc.0.max(chunk[0]), acc.1.max(chunk[1]))
    });
    let mut field = vec![vec![b' '; max_x as usize + 1]; max_y as usize + 1];
    let mut prev_ball = (0, 0);
    let mut ball = (0, 0);
    let mut pos = (0, 0);
    let mut result = 0;
    while !int_code.output.is_empty() {
        let mut i = 0;
        while i < int_code.output.len() {
            let chunk = &int_code.output[i..i + 3];
            match chunk {
                [-1, 0, points] => result = *points,
                [x, y, 0] => field[*y as usize][*x as usize] = b' ',
                [x, y, 1] => field[*y as usize][*x as usize] = b'#',
                [x, y, 2] => field[*y as usize][*x as usize] = b'*',
                [x, y, 3] => {
                    field[*y as usize][*x as usize] = b'-';
                    pos = (*x, *y);
                }
                [x, y, 4] => {
                    field[*y as usize][*x as usize] = b'o';
                    prev_ball = ball;
                    ball = (*x, *y);
                }
                _ => panic!("Unexpected"),
            }
            i += 3;
        }
        int_code.output.clear();
        let target_x = if ball.1 + 1 == pos.1 {
            ball.0
        } else if ball.0 > prev_ball.0 {
            ball.0 + 1
        } else {
            ball.0 - 1
        };
        if target_x > pos.0 {
            int_code.add_input(1);
        } else if target_x < pos.0 {
            int_code.add_input(-1);
        } else {
            int_code.add_input(0);
        }

        int_code.execute();
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day13_part1() {}

    #[test]
    fn test_day13_part2() {}
}
