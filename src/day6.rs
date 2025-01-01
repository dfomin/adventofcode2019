use std::collections::HashMap;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .fold(HashMap::new(), |mut map: HashMap<&str, Vec<&str>>, line| {
            let mut iter = line.trim().split(")");
            let first = iter.next().unwrap();
            let second = iter.next().unwrap();
            map.entry(first).or_default().push(second);
            map
        })
}

pub fn part1(input: &str) -> i64 {
    let map = parse(input);
    let mut queue: Vec<(&str, i64)> = vec![("COM", 0)];
    let mut result = 0;
    while let Some((name, depth)) = queue.pop() {
        result += depth;
        map.get(name).unwrap_or(&vec![]).iter().for_each(|x| {
            queue.push((x, depth + 1));
        });
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let map = parse(input);
    let mut queue: Vec<(&str, i64, Vec<&str>)> = vec![("COM", 0, vec!["COM"])];
    let mut you: Vec<&str> = vec![];
    let mut san: Vec<&str> = vec![];
    while let Some((name, depth, path)) = queue.pop() {
        if name == "YOU" {
            you = path.clone();
        } else if name == "SAN" {
            san = path.clone();
        }
        map.get(name).unwrap_or(&vec![]).iter().for_each(|x| {
            let mut new_path = path.clone();
            new_path.push(x);
            queue.push((x, depth + 1, new_path));
        });
    }
    for i in 0..you.len().min(san.len()) {
        if you[i] != san[i] {
            return (you.len() - i + san.len() - i) as i64 - 2;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";

    const INPUT2: &str = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";

    #[test]
    fn test_day6_part1() {
        assert_eq!(part1(INPUT1), 42);
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(part2(INPUT2), 4);
    }
}
