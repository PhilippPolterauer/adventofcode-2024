use std::collections::{HashMap, HashSet};

use adventofcode2024::matrix::{
    Direction, FromChar, Matrix, MatrixElement, MatrixIdx, MatrixIdxOffset,
};
use adventofcode2024::util;

#[derive(Debug, Clone, PartialEq)]
enum LabTile {
    Tile,
    Obstacle,
    Guard,
}
impl FromChar for LabTile {
    fn try_from_char(char: &char) -> Option<Self> {
        match char {
            '#' => Some(LabTile::Obstacle),
            '^' => Some(LabTile::Guard),
            '.' => Some(LabTile::Tile),
            _ => None,
        }
    }
}
impl MatrixElement for LabTile {}
fn idx_offset(dir: &Direction) -> MatrixIdxOffset {
    use Direction::*;
    let (rows, cols) = match dir {
        Up => (-1, 0),
        Down => (1, 0),
        Left => (0, -1),
        Right => (0, 1),
    };
    MatrixIdxOffset::new(rows, cols)
}

//fn trace_path(matrix: &Matrix

fn part1(content: &str) -> usize {
    use LabTile::*;
    let floor = Matrix::<LabTile>::try_from_str(content).expect("parsing into matrix failed");
    let start = floor.find(&LabTile::Guard);
    let mut path = HashMap::new();
    if let Some(mut current) = start {
        let mut dir = Direction::Up;

        loop {
            // first we update the state of the machine

            let tile = floor.get(&current);
            //dbg!(&tile, &current, &dir);
            match tile {
                Some(tile) => match tile {
                    Tile | Guard => {
                        let val = path.entry(current).or_insert(Vec::new());
                        val.push(dir);
                    }
                    Obstacle => panic!("this should never happen"),
                },
                None => break,
            }

            current = loop {
                let next = current + idx_offset(&dir);
                match floor.get(&next) {
                    Some(Obstacle) => dir = dir.right(),
                    _ => break next,
                }
            };
        }
    }
    path.len()
}

enum CheckPathResult {
    Loop,
    Exit,
}
fn check_path_loop(floor: &Matrix<LabTile>, start: MatrixIdx, dir: Direction) -> CheckPathResult {
    use LabTile::*;
    let mut path = HashMap::new();
    let mut dir = dir;
    let mut current = start;
    loop {
        // first we update the state of the machine

        let tile = floor.get(&current);
        match tile {
            Some(tile) => match tile {
                Tile | Guard => {
                    let val = path.entry(current).or_insert(Vec::new());
                    if val.contains(&dir) {
                        // loop was found if the path already contains the direction
                        return CheckPathResult::Loop;
                    }
                    val.push(dir);
                }
                Obstacle => panic!("this should never happen"),
            },
            None => break,
        }

        current = loop {
            let next = current + idx_offset(&dir);
            match floor.get(&next) {
                Some(Obstacle) => dir = dir.right(),
                _ => break next,
            }
        };
    }
    CheckPathResult::Exit
}

fn part2(content: &str) -> usize {
    use LabTile::*;
    let floor = Matrix::<LabTile>::try_from_str(content).expect("parsing into matrix failed");
    let start = floor.find(&LabTile::Guard).expect("start point not found");
    let mut path = HashMap::new();
    let mut dir = Direction::Up;
    let mut current = start;
    let mut loop_cnt = 0;
    let mut loop_obstructions = HashSet::new();
    loop {
        // first we update the state of the machine

        let tile = floor.get(&current);
        //dbg!(&tile, &current, &dir);
        match tile {
            Some(tile) => match tile {
                Tile | Guard => {
                    let val = path.entry(current).or_insert(Vec::new());
                    val.push(dir);
                }
                Obstacle => panic!("this should never happen"),
            },
            None => break,
        }

        let mut next;
        current = loop {
            next = current + idx_offset(&dir);
            match floor.get(&next) {
                Some(Obstacle) => dir = dir.right(),
                Some(Tile | Guard) => {
                    // here we spawn a loop check with once to the right but only if this tile has
                    // not been visited before
                    //if !path.contains_key(&next) {
                    // we can test for a loop by inserting a block as this would not alter the
                    // previous path generation
                    match check_path_loop(&floor, current, dir.right()) {
                        CheckPathResult::Loop => {
                            loop_cnt += 1;
                            loop_obstructions.insert(next);
                        }
                        _ => (),
                    }
                    //}
                    break next;
                }
                None => break next,
            }
        };
    }
    dbg!(&loop_obstructions, loop_cnt);
    loop_obstructions.len()
}

fn main() {
    let content = util::load_file(6, 1, true).expect("failed to load input text file");
    let solution = part1(&content);
    dbg!(solution);

    let solution = part2(&content);
    dbg!(solution);
}
