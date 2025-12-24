use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    i64,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse(input: &str, change: bool) -> (Vec<Vec<u8>>, Pos, u32, HashMap<u8, Pos>) {
    let mut field: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes().iter().cloned().collect())
        .collect();
    let mut start = Pos { x: 0, y: 0 };
    let mut count = 0;
    let mut keys = HashMap::new();
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            let ch = field[i][j];
            if ch == b'@' {
                start = Pos { x: j, y: i };
            } else if ch.is_ascii_lowercase() {
                keys.insert(ch, Pos { x: j, y: i });
            }
            if ch.is_ascii_lowercase() {
                count += 1;
            }
        }
    }

    if change {
        field[start.y - 1][start.x - 1] = b'@';
        field[start.y - 1][start.x + 1] = b'@';
        field[start.y + 1][start.x - 1] = b'@';
        field[start.y + 1][start.x + 1] = b'@';
        field[start.y][start.x - 1] = b'#';
        field[start.y][start.x + 1] = b'#';
        field[start.y - 1][start.x] = b'#';
        field[start.y + 1][start.x] = b'#';
        field[start.y][start.x] = b'#';
    }

    (field, start, count, keys)
}

fn bfs(field: &[Vec<u8>], pos: Pos, keys: u64) -> HashMap<u8, i64> {
    let mut result = HashMap::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((pos, 0));
    while let Some((pos, step)) = queue.pop_front() {
        let ch = field[pos.y][pos.x];
        visited.insert(pos);
        if ch.is_ascii_lowercase() {
            let key_index = ch as u8 - b'a';
            if keys & (1 << key_index) == 0 {
                let cur_result = *result.get(&ch).unwrap_or(&i64::MAX);
                result.insert(ch, cur_result.min(step));
            }
        }
        for shift in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = (pos.x as i64 + shift.0) as usize;
            let y = (pos.y as i64 + shift.1) as usize;
            if field[y][x] == b'#' {
                continue;
            }
            if field[y][x].is_ascii_uppercase() {
                let key_index = field[y][x] as u8 - b'A';
                if keys & (1 << key_index) == 0 {
                    continue;
                }
            }
            let next_pos = Pos { x, y };
            if visited.contains(&next_pos) {
                continue;
            }
            queue.push_back((next_pos, step + 1));
        }
    }
    result
}

pub fn part1(input: &str) -> i64 {
    let (field, pos, count, _) = parse(input, false);
    let mut cache: HashMap<(Pos, u64), i64> = HashMap::new();
    cache.insert((pos, 0), 0);
    let mut queue = VecDeque::new();
    queue.push_back((0, pos, 0));
    let mut result = i64::MAX;
    while let Some((cur_score, cur_pos, cur_keys)) = queue.pop_front() {
        if let Some(&prev_score) = cache.get(&(cur_pos, cur_keys))
            && (prev_score < cur_score || cur_score >= result)
        {
            continue;
        }

        cache.insert((cur_pos, cur_keys), cur_score);

        for shift in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let x = (cur_pos.x as i64 + shift.0) as usize;
            let y = (cur_pos.y as i64 + shift.1) as usize;
            if field[y][x] == b'#' {
                continue;
            }
            if field[y][x].is_ascii_uppercase() {
                let key_index = field[y][x] as u8 - b'A';
                if cur_keys & (1 << key_index) == 0 {
                    continue;
                }
            }
            let next_score = cur_score + 1;
            let mut next_keys = cur_keys;
            if field[y][x].is_ascii_lowercase() {
                let key_index = field[y][x] as u8 - b'a';
                next_keys |= 1 << key_index;
            }
            if next_keys.count_ones() == count {
                result = result.min(next_score);
                continue;
            }
            let next_pos = Pos { x, y };
            if let Some(&prev_score) = cache.get(&(next_pos, next_keys))
                && (prev_score <= next_score || next_score >= result)
            {
                continue;
            }

            cache.insert((next_pos, next_keys), next_score);
            queue.push_back((next_score, next_pos, next_keys));
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let (field, pos, count, keys) = parse(input, true);
    let mut cache: HashMap<([Pos; 4], u64), i64> = HashMap::new();
    let positions = [
        Pos {
            x: pos.x - 1,
            y: pos.y - 1,
        },
        Pos {
            x: pos.x + 1,
            y: pos.y - 1,
        },
        Pos {
            x: pos.x - 1,
            y: pos.y + 1,
        },
        Pos {
            x: pos.x + 1,
            y: pos.y + 1,
        },
    ];
    cache.insert((positions, 0), 0);
    let mut queue = BinaryHeap::new();
    queue.push((0, positions, 0));
    let mut result = i64::MAX;
    while let Some((mut cur_score, cur_positions, cur_keys)) = queue.pop() {
        cur_score = -cur_score;
        if let Some(&prev_score) = cache.get(&(cur_positions, cur_keys))
            && (prev_score < cur_score || cur_score >= result)
        {
            continue;
        }

        cache.insert((cur_positions, cur_keys), cur_score);

        for i in 0..4 {
            for (ch, steps) in bfs(&field, cur_positions[i], cur_keys) {
                let next_pos = keys[&ch];
                let mut next_positions = cur_positions.clone();
                next_positions[i] = next_pos;

                let next_score = cur_score + steps;

                let mut next_keys = cur_keys;
                let key_index = ch - b'a';
                next_keys |= 1 << key_index;

                if next_keys.count_ones() == count {
                    result = result.min(next_score);
                    continue;
                }

                if let Some(&prev_score) = cache.get(&(next_positions, next_keys))
                    && (prev_score <= next_score || next_score >= result)
                {
                    continue;
                }

                cache.insert((next_positions, next_keys), next_score);
                queue.push((-next_score, next_positions, next_keys));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        #########
        #b.A.@.a#
        #########
    ";

    const INPUT2: &str = "
        ########################
        #f.D.E.e.C.b.A.@.a.B.c.#
        ######################.#
        #d.....................#
        ########################";

    const INPUT3: &str = "
        ########################
        #...............b.C.D.f#
        #.######################
        #.....@.a.B.c.d.A.e.F.g#
        ########################
    ";

    const INPUT4: &str = "
        #################
        #i.G..c...e..H.p#
        ########.########
        #j.A..b...f..D.o#
        ########@########
        #k.E..a...g..B.n#
        ########.########
        #l.F..d...h..C.m#
        #################
    ";

    const INPUT5: &str = "
        ########################
        #@..............ac.GI.b#
        ###d#e#f################
        ###A#B#C################
        ###g#h#i################
        ########################
    ";

    const INPUT6: &str = "
        #######
        #a.#Cd#
        ##...##
        ##.@.##
        ##...##
        #cB#Ab#
        #######
    ";

    const INPUT7: &str = "
        ###############
        #d.ABC.#.....a#
        ######...######
        ######.@.######
        ######...######
        #b.....#.....c#
        ###############
    ";

    const INPUT8: &str = "
        #############
        #DcBa.#.GhKl#
        #.###...#I###
        #e#d#.@.#j#k#
        ###C#...###J#
        #fEbA.#.FgHi#
        #############
    ";

    const INPUT9: &str = "
        #############
        #g#f.D#..h#l#
        #F###e#E###.#
        #dCba...BcIJ#
        #####.@.#####
        #nK.L...G...#
        #M###N#H###.#
        #o#m..#i#jk.#
        #############
    ";

    #[test]
    fn test_day18_part1() {
        assert_eq!(part1(INPUT1), 8);
        assert_eq!(part1(INPUT2), 86);
        assert_eq!(part1(INPUT3), 132);
        assert_eq!(part1(INPUT4), 136);
        assert_eq!(part1(INPUT5), 81);
    }

    #[test]
    fn test_day18_part2() {
        assert_eq!(part2(INPUT6), 8);
        assert_eq!(part2(INPUT7), 24);
        assert_eq!(part2(INPUT8), 32);
        assert_eq!(part2(INPUT9), 72);
    }
}
