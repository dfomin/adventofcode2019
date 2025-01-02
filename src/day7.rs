use std::collections::VecDeque;

use itertools::Itertools;

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>()
}

fn value(values: &[i64], index: usize, step: usize, modes: i64) -> usize {
    if modes / 10i64.pow(step as u32 - 1) % 10 == 0 {
        values[index] as usize
    } else {
        index
    }
}

fn execute(mut values: Vec<i64>, mut input: VecDeque<i64>) -> i64 {
    let mut index = 0usize;
    let mut output = 0;
    while values[index] % 100 != 99 {
        let op = values[index] % 100;
        let modes = values[index] / 100;

        let op_count = match op {
            1 | 2 | 7 | 8 => 4,
            3 | 4 => 2,
            5 | 6 => 3,
            _ => panic!("Unexpected command: {}", values[index]),
        };

        let ops = (1..op_count)
            .map(|i| value(&values, index + i, i, modes))
            .collect::<Vec<_>>();
        match op {
            1 => {
                values[ops[2]] = values[ops[0]] + values[ops[1]];
                index += 4;
            }
            2 => {
                values[ops[2]] = values[ops[0]] * values[ops[1]];
                index += 4;
            }
            3 => {
                values[ops[0]] = input.pop_front().unwrap();
                index += 2;
            }
            4 => {
                output = values[ops[0]];
                index += 2;
            }
            5 => {
                if values[ops[0]] != 0 {
                    index = values[ops[1]] as usize;
                } else {
                    index += 3;
                }
            }
            6 => {
                if values[ops[0]] == 0 {
                    index = values[ops[1]] as usize;
                } else {
                    index += 3;
                }
            }
            7 => {
                if values[ops[0]] < values[ops[1]] {
                    values[ops[2]] = 1;
                } else {
                    values[ops[2]] = 0;
                }
                index += 4;
            }
            8 => {
                if values[ops[0]] == values[ops[1]] {
                    values[ops[2]] = 1;
                } else {
                    values[ops[2]] = 0;
                }
                index += 4;
            }
            _ => panic!("Unexpected command: {}", values[index]),
        }
    }
    output
}

fn solve(input: &str, low: i64, high: i64, repeat: bool) -> i64 {
    let values = parse(input);
    let mut result = 0;
    for phases in (low..high).permutations((high - low) as usize) {
        let mut output = 0;
        for phase in phases {
            output = execute(values.clone(), VecDeque::from([phase, output]));
        }
        result = result.max(output);
    }
    result
}

pub fn part1(input: &str) -> i64 {
    solve(input, 0, 5, false)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 5, 10, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        assert_eq!(
            part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );

        assert_eq!(
            part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
            54321
        );

        assert_eq!(
            part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
            65210
        );
    }

    #[test]
    fn test_day7_part2() {
        assert_eq!(
            part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
            139629729
        );

        assert_eq!(
            part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
            18216
        );
    }
}
