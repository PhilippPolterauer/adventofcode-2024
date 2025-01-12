use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

use adventofcode2024::{
    matrix::{Direction, Matrix, MatrixIdx, MatrixIdxOffset},
    util,
};

fn find_plots(grid: &Matrix<char>) -> Vec<HashSet<MatrixIdx>> {
    let mut found = HashSet::<MatrixIdx>::new();
    let mut plots = Vec::new();
    for start in grid.indizes() {
        if !found.contains(&start) {
            let plot = find_plot(grid, start);
            found.extend(&plot);
            plots.push(plot);
        }
    }
    plots
}
fn find_plot(grid: &Matrix<char>, start: MatrixIdx) -> HashSet<MatrixIdx> {
    use Direction::*;
    let kind = grid[start];
    let mut plot = HashSet::new();
    plot.insert(start);
    let mut front = VecDeque::from([start]);
    while let Some(tile) = front.pop_front() {
        for dir in [Up, Right, Down, Left] {
            let next_idx = tile + offset(&dir);
            if let Some(next) = grid.get(&next_idx) {
                if *next == kind {
                    if plot.insert(next_idx) {
                        front.push_back(next_idx);
                    }
                }
            }
        }
    }
    plot
}
fn find_perimeter(plot: &HashSet<MatrixIdx>) -> usize {
    use Direction::*;
    let mut perimeter = 0;
    for tile in plot {
        for dir in [Up, Right, Down, Left] {
            let next_idx = tile + &offset(&dir);
            if !plot.contains(&next_idx) {
                perimeter += 1;
            }
        }
    }
    perimeter
}

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
fn part1(content: &str) -> usize {
    let grid = Matrix::<char>::try_from_str_with(content, |c| Some(*c)).unwrap();
    let mut solution = 0;
    let plots = find_plots(&grid);
    for p in plots {
        solution += p.len() * find_perimeter(&p);
    }
    solution
}
fn part2(content: &str) -> usize {
    let stones: Vec<usize> = content
        .split(' ')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    let mut solution = 0;
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mem() {}
}
