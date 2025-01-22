use std::{
    collections::{vec_deque, HashMap, HashSet, VecDeque},
    hash::Hash,
};

use adventofcode2024::{
    matrix::{Direction, MatrixIdx, MatrixIdxOffset, ALL_DIRECTIONS},
    util,
};
fn offset(dir: &Direction) -> MatrixIdxOffset {
    use Direction::*;
    let (rows, cols) = match dir {
        Up => (-1, 0),
        Right => (0, 1),
        Down => (1, 0),
        Left => (0, -1),
    };
    MatrixIdxOffset::new(rows, cols)
}
fn parse(content: &str) -> Vec<MatrixIdx> {
    content
        .lines()
        .filter_map(|line| {
            line.split_once(",")
                .and_then(|(a, b)| Some(MatrixIdx::new(b.parse().unwrap(), a.parse().unwrap())))
        })
        .collect()
}
fn part1(content: &str) -> usize {
    let corrupted: HashSet<MatrixIdx> = parse(content).into_iter().take(1024).collect();

    let start = MatrixIdx::new(0, 0);
    let width = 71;
    let height = 71;
    let mut front = VecDeque::from([start]);
    let mut costmap = HashMap::new();
    costmap.insert(start, 0usize);
    while let Some(pos) = front.pop_front() {
        let cost = costmap[&pos] + 1;
        for dir in ALL_DIRECTIONS {
            let next = pos + offset(&dir);
            if next.row < height && next.col < width && !corrupted.contains(&next) {
                costmap
                    .entry(next)
                    .and_modify(|val| {
                        if cost < *val {
                            *val = cost;
                            front.push_back(next);
                        }
                    })
                    .or_insert_with(|| {
                        front.push_back(next);
                        cost
                    });
            }
        }
    }
    costmap[&MatrixIdx::new(height - 1, width - 1)]
}
fn part2(content: &str) -> usize {
    let all_corrupted = parse(content);
    let start = MatrixIdx::new(0, 0);
    let width = 71;
    let height = 71;
    let end = MatrixIdx::new(height - 1, width - 1);

    for i in 1..all_corrupted.len() {
        let mut visited = HashSet::from([start]);
        let mut front = VecDeque::from([start]);
        let mut connected = false;
        'outer: while let Some(pos) = front.pop_front() {
            for dir in ALL_DIRECTIONS {
                let next = pos + offset(&dir);
                if next == end {
                    dbg!("end");
                    connected = true;
                    break 'outer;
                }
                if !visited.contains(&next)
                    && next.row < height
                    && next.col < width
                    && !all_corrupted[0..i + 1].contains(&next)
                {
                    front.push_back(next);
                    visited.insert(next);
                }
            }
        }
        if !connected {
            dbg!(all_corrupted[i]);
            break;
        } else {
            dbg!(i, front);
        }
    }
    0
}

fn main() {
    let content =
        util::load_file(util::get_day(), 1, false).expect("failed to load input text file");
    let solution = part1(&content);
    dbg!(solution);
    let solution = part2(&content);
    dbg!(solution);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {}
}
