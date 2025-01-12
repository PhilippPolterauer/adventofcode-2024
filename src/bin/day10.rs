use std::collections::HashSet;

use adventofcode2024::{
    matrix::{Direction, Matrix, MatrixElement, MatrixIdx, MatrixIdxOffset},
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

fn find_heads(start: &MatrixIdx, grid: &Matrix<u8>, dir: &Direction) -> Vec<MatrixIdx> {
    let next_idx = start + &offset(dir);
    let prev = grid.get(&start);
    let next = grid.get(&next_idx);
    match (prev, next) {
        (Some(prev), Some(next)) => {
            if *next == prev + 1 {
                if *next == 9 {
                    vec![next_idx]
                } else {
                    vec![
                        find_heads(&next_idx, &grid, &Direction::Up),
                        find_heads(&next_idx, &grid, &Direction::Down),
                        find_heads(&next_idx, &grid, &Direction::Left),
                        find_heads(&next_idx, &grid, &Direction::Right),
                    ]
                    .concat()
                }
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

fn find_trails(start: &MatrixIdx, grid: &Matrix<u8>, dir: &Direction) -> usize {
    let next_idx = start + &offset(dir);
    let prev = grid.get(&start);
    let next = grid.get(&next_idx);
    match (prev, next) {
        (Some(prev), Some(next)) => {
            if *next == prev + 1 {
                if *next == 9 {
                    1
                } else {
                    find_trails(&next_idx, &grid, &Direction::Up)
                        + find_trails(&next_idx, &grid, &Direction::Down)
                        + find_trails(&next_idx, &grid, &Direction::Left)
                        + find_trails(&next_idx, &grid, &Direction::Right)
                }
            } else {
                0
            }
        }
        _ => 0,
    }
}
fn part2(content: &str) -> usize {
    let mut solution = 0;
    let grid = Matrix::<u8>::try_from_str_with(content, |c| c.to_digit(10).map(|c| c as u8))
        .expect("parsing matrix failed");

    let starts = grid.find_all(&0);
    for start in starts {
        solution += find_trails(&start, &grid, &Direction::Up)
            + find_trails(&start, &grid, &Direction::Right)
            + find_trails(&start, &grid, &Direction::Down)
            + find_trails(&start, &grid, &Direction::Left)
    }
    solution
}
fn part1(content: &str) -> usize {
    let mut solution = 0;
    let grid = Matrix::<u8>::try_from_str_with(content, |c| c.to_digit(10).map(|c| c as u8))
        .expect("parsing matrix failed");

    let starts = grid.find_all(&0);
    for start in starts {
        let heads: HashSet<MatrixIdx> = vec![
            find_heads(&start, &grid, &Direction::Up),
            find_heads(&start, &grid, &Direction::Right),
            find_heads(&start, &grid, &Direction::Down),
            find_heads(&start, &grid, &Direction::Left),
        ]
        .concat()
        .into_iter()
        .collect();
        let score = heads.len();
        solution += score;
    }
    solution
}

fn main() {
    let content =
        util::load_file(util::get_day(), 1, false).expect("failed to load input text file");

    let solution = part1(&content);
    dbg!(solution);
    let solution = part2(&content);
    dbg!(solution);
}
