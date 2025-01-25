use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug, Clone, Copy)]
enum CellType {
    Empty,
    Wall,
    Oxygen,
}

#[derive(Default)]
struct Field {
    points: HashMap<(i64, i64), CellType>,
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

impl Field {
    fn add_cell(&mut self, x: i64, y: i64, cell_type: CellType) {
        self.points.insert((x, y), cell_type);
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.min_y..self.max_y + 1 {
            for j in self.min_x..self.max_x + 1 {
                if let Some(&cell_type) = self.points.get(&(j, i)) {
                    match cell_type {
                        CellType::Empty => write!(f, "{}", '.')?,
                        CellType::Wall => write!(f, "{}", '#')?,
                        CellType::Oxygen => write!(f, "{}", '!')?,
                    }
                } else {
                    write!(f, "{}", ' ')?;
                }
            }
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}

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

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>()
}

pub fn part1(input: &str) -> i64 {
    let values = parse(input);
    let mut field = Field::default();
    let dirs = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut pool = VecDeque::new();
    pool.push_back((0, (0, 0), IntCode::new(values, VecDeque::new())));
    while let Some((steps, pos, int_code)) = pool.pop_front() {
        for direction in 1..5 {
            let next_pos = (
                pos.0 + dirs[direction as usize - 1].0,
                pos.1 + dirs[direction as usize - 1].1,
            );
            if visited.contains(&next_pos) {
                continue;
            }
            let mut clone_int_code = int_code.clone();
            clone_int_code.add_input(direction);
            clone_int_code.execute();
            visited.insert(next_pos);
            match *clone_int_code.output.last().unwrap() {
                0 => {
                    field.add_cell(next_pos.0, next_pos.1, CellType::Wall);
                }
                1 => {
                    field.add_cell(next_pos.0, next_pos.1, CellType::Empty);
                    pool.push_back((steps + 1, next_pos, clone_int_code.clone()));
                }
                2 => {
                    return steps + 1;
                }
                _ => panic!("Unexpected"),
            }
        }
    }
    0
}

pub fn part2(input: &str) -> i64 {
    let values = parse(input);
    let mut field = Field::default();
    let dirs = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut pool = VecDeque::new();
    pool.push_back((0, (0, 0), IntCode::new(values, VecDeque::new())));
    let mut oxygen = HashSet::new();
    while let Some((steps, pos, int_code)) = pool.pop_front() {
        for direction in 1..5 {
            let next_pos = (
                pos.0 + dirs[direction as usize - 1].0,
                pos.1 + dirs[direction as usize - 1].1,
            );
            if visited.contains(&next_pos) {
                continue;
            }
            let mut clone_int_code = int_code.clone();
            clone_int_code.add_input(direction);
            clone_int_code.execute();
            visited.insert(next_pos);
            match *clone_int_code.output.last().unwrap() {
                0 => {
                    field.add_cell(next_pos.0, next_pos.1, CellType::Wall);
                }
                1 => {
                    field.add_cell(next_pos.0, next_pos.1, CellType::Empty);
                    pool.push_back((steps + 1, next_pos, clone_int_code.clone()));
                }
                2 => {
                    field.add_cell(next_pos.0, next_pos.1, CellType::Oxygen);
                    oxygen.insert(next_pos);
                }
                _ => panic!("Unexpected"),
            }
        }
    }
    let mut steps = 0;
    let mut frontier = Vec::from_iter(oxygen.iter().cloned());
    while !frontier.is_empty() {
        let mut next_frontier = vec![];
        for pos in frontier {
            for dir in 0..4 {
                let next_pos = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
                if !matches!(field.points.get(&next_pos), Some(CellType::Empty)) {
                    continue;
                }
                if oxygen.contains(&next_pos) {
                    continue;
                }
                next_frontier.push(next_pos);
                oxygen.insert(next_pos);
            }
        }
        steps += 1;
        frontier = next_frontier;
    }
    steps - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day15_part1() {}

    #[test]
    fn test_day15_part2() {}
}
