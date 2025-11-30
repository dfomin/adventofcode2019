use std::{collections::VecDeque, io::BufRead};

#[derive(Debug, Clone)]
enum IntCodeState {
    Created,
    WaitInput,
    Halted,
}

#[derive(Clone)]
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

struct Pos {
    x: usize,
    y: usize,
    dir: usize,
}

impl Pos {
    fn step(&self, field: &[Vec<u8>], dir: usize) -> Option<(Pos, u8)> {
        let width = field[0].len() as i64;
        let height = field.len() as i64;
        let dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)];
        let new_x = self.x as i64 + dirs[dir].0;
        let new_y = self.y as i64 + dirs[dir].1;
        if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
            return None;
        }
        let pos = Pos {
            x: new_x as usize,
            y: new_y as usize,
            dir,
        };
        let value = field[pos.y][pos.x];
        Some((pos, value))
    }

    fn straight(&self, field: &[Vec<u8>]) -> Option<(Pos, u8)> {
        self.step(field, self.dir)
    }

    fn left(&self, field: &[Vec<u8>]) -> Option<(Pos, u8)> {
        self.step(field, (self.dir + 3) % 4)
    }

    fn right(&self, field: &[Vec<u8>]) -> Option<(Pos, u8)> {
        self.step(field, (self.dir + 1) % 4)
    }
}

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_field(input: &str) -> Vec<Vec<u8>> {
    let values = parse(input);
    let mut int_code = IntCode::new(values, VecDeque::new());
    int_code.execute();
    int_code
        .output
        .iter()
        .map(|&value| value as u8)
        .collect::<Vec<_>>()
        .split(|&ch| ch == b'\n')
        .map(|chunk| chunk.to_vec())
        .filter(|vec| !vec.is_empty())
        .collect::<Vec<_>>()
}

fn check(path: &[i64], l1: &[i64], l2: &[i64], l3: &[i64]) -> Option<Vec<i64>> {
    let mut index = 0;
    let mut result = Vec::new();
    while index < path.len() {
        if index + l1.len() <= path.len() && l1 == &path[index..index + l1.len()] {
            result.push(0);
            index += l1.len();
        } else if index + l2.len() <= path.len() && l2 == &path[index..index + l2.len()] {
            result.push(1);
            index += l2.len();
        } else if index + l3.len() <= path.len() && l3 == &path[index..index + l3.len()] {
            result.push(2);
            index += l3.len();
        } else {
            return None;
        }
    }
    Some(result)
}

fn find_routines(path: &[i64]) -> (Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>) {
    for l1_size in 6..11 {
        for l2_size in 6..11 {
            for l3_size in 6..11 {
                let l1 = path[..l1_size].to_vec();
                for l2_start in (l1_size..path.len()).step_by(l1_size) {
                    if l2_start + l2_size >= path.len() {
                        break;
                    }
                    let l2 = path[l2_start..l2_start + l2_size].to_vec();
                    for l3_start in l2_start + l2_size..path.len() {
                        if l3_start + l3_size >= path.len() {
                            break;
                        }
                        let l3 = path[l3_start..l3_start + l3_size].to_vec();
                        if let Some(result) = check(&path, &l1, &l2, &l3) {
                            return (result, l1, l2, l3);
                        }
                    }
                }
            }
        }
    }
    (Vec::new(), Vec::new(), Vec::new(), Vec::new())
}

pub fn part1(input: &str) -> i64 {
    let field = parse_field(input);
    let (width, height) = (field[0].len(), field.len());
    let mut result = 0;
    let dirs = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if dirs.iter().all(|dir| {
                match field[(i as i64 + dir.0) as usize][(j as i64 + dir.1) as usize] {
                    b'#' | b'<' | b'>' | b'^' | b'v' => true,
                    _ => false,
                }
            }) {
                result += (i * j) as i64;
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let field = parse_field(input);
    let (width, height) = (field[0].len(), field.len());
    let mut pos = Pos { x: 0, y: 0, dir: 0 };
    for i in 0..height {
        for j in 0..width {
            if field[i][j] == b'v' {
                pos = Pos { x: j, y: i, dir: 0 }
            } else if field[i][j] == b'<' {
                pos = Pos { x: j, y: i, dir: 1 }
            } else if field[i][j] == b'^' {
                pos = Pos { x: j, y: i, dir: 2 }
            } else if field[i][j] == b'>' {
                pos = Pos { x: j, y: i, dir: 3 }
            }
        }
    }
    let mut path = Vec::new();
    let mut cur = 0;
    loop {
        if let Some((new_pos, value)) = pos.straight(&field)
            && value == b'#'
        {
            cur += 1;
            pos = new_pos;
        } else if let Some((new_pos, value)) = pos.left(&field)
            && value == b'#'
        {
            if cur > 0 {
                path.push(cur);
            }
            path.push(-1);
            cur = 1;
            pos = new_pos;
        } else if let Some((new_pos, value)) = pos.right(&field)
            && value == b'#'
        {
            if cur > 0 {
                path.push(cur);
            }
            path.push(-2);
            cur = 1;
            pos = new_pos;
        } else {
            path.push(cur);
            break;
        }
    }
    let (main, a, b, c) = find_routines(&path);
    let mut values = parse(input);
    values[0] = 2;
    let mut int_code = IntCode::new(values, VecDeque::new());
    for i in 0..main.len() {
        if main[i] == 0 {
            int_code.add_input(b'A' as i64);
        } else if main[i] == 1 {
            int_code.add_input(b'B' as i64);
        } else if main[i] == 2 {
            int_code.add_input(b'C' as i64);
        }

        if i < main.len() - 1 {
            int_code.add_input(b',' as i64);
        } else {
            int_code.add_input(10);
        }
    }
    for routine in [a, b, c] {
        for i in 0..routine.len() {
            if routine[i] == -1 {
                int_code.add_input(b'L' as i64);
            } else if routine[i] == -2 {
                int_code.add_input(b'R' as i64);
            } else {
                let mut value = routine[i];
                let mut chars = vec![];
                while value > 0 {
                    chars.push(b'0' as i64 + value % 10);
                    value /= 10;
                }
                for char in chars.into_iter().rev() {
                    int_code.add_input(char);
                }
            }

            if i < routine.len() - 1 {
                int_code.add_input(b',' as i64);
            } else {
                int_code.add_input(10);
            }
        }
    }
    int_code.add_input(b'n' as i64);
    int_code.add_input(10);
    int_code.execute();
    // println!(
    //     "{}",
    //     String::from_utf8(
    //         int_code.output[..int_code.output.len() - 1]
    //             .iter()
    //             .map(|x| *x as u8)
    //             .collect()
    //     )
    //     .unwrap()
    // );
    *int_code.output.last().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day17_part1() {}

    #[test]
    fn test_day17_part2() {}
}
