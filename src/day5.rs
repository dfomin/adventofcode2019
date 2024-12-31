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

fn execute(mut values: Vec<i64>, input: i64) -> i64 {
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
                values[ops[0]] = input;
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

pub fn part1(input: &str) -> i64 {
    let values = parse(input);
    execute(values, 1)
}

pub fn part2(input: &str) -> i64 {
    let values = parse(input);
    execute(values, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1() {}

    #[test]
    fn test_day5_part2() {
        assert_eq!(execute(parse("3,9,8,9,10,9,4,9,99,-1,8"), 8), 1);
        assert_eq!(execute(parse("3,9,8,9,10,9,4,9,99,-1,8"), 9), 0);

        assert_eq!(execute(parse("3,9,7,9,10,9,4,9,99,-1,8"), 7), 1);
        assert_eq!(execute(parse("3,9,7,9,10,9,4,9,99,-1,8"), 8), 0);

        assert_eq!(execute(parse("3,3,1108,-1,8,3,4,3,99"), 8), 1);
        assert_eq!(execute(parse("3,3,1108,-1,8,3,4,3,99"), 10), 0);

        assert_eq!(execute(parse("3,3,1107,-1,8,3,4,3,99"), 7), 1);
        assert_eq!(execute(parse("3,3,1107,-1,8,3,4,3,99"), 8), 0);

        assert_eq!(
            execute(parse("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"), 0),
            0
        );
        assert_eq!(
            execute(parse("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"), 5),
            1
        );

        assert_eq!(execute(parse("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), 0), 0);
        assert_eq!(execute(parse("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), 5), 1);

        assert_eq!(
            execute(
                parse(
                    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
                ),
                7
            ),
            999
        );
        assert_eq!(
            execute(
                parse(
                    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
                ),
                8
            ),
            1000
        );
        assert_eq!(
            execute(
                parse(
                    "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
                ),
                9
            ),
            1001
        );
    }
}
