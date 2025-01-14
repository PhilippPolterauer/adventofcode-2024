use std::collections::{HashMap, HashSet};

use adventofcode2024::matrix::{Direction, FromChar, Matrix, MatrixIdx, MatrixIdxOffset};
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

fn part1(content: &str) -> (usize, Vec<(MatrixIdx, Direction)>) {
    use LabTile::*;
    let floor = Matrix::<LabTile>::try_from_str(content).expect("parsing into matrix failed");
    let start = floor.find(&LabTile::Guard);
    let mut visited = HashMap::new();
    let mut path = Vec::new();
    if let Some(mut current) = start {
        let mut dir = Direction::Up;

        loop {
            // first we update the state of the machine

            let tile = floor.get(&current);
            match tile {
                Some(tile) => match tile {
                    Tile | Guard => {
                        let val = visited.entry(current).or_insert(Vec::new());
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
                    _ => {
                        path.push((current, dir));
                        break next;
                    }
                }
            };
        }
    }
    (visited.len(), path)
}

fn path_has_loop(floor: &Matrix<LabTile>, start: MatrixIdx, dir: Direction) -> bool {
    use LabTile::*;
    let mut path = Vec::new();
    let mut dir = dir;
    let mut current = start;
    let obstacle_location = start + idx_offset(&dir.left());
    loop {
        // first we update the state of the machine

        let tile = floor.get(&current);
        match tile {
            Some(tile) => match tile {
                Tile | Guard => {}
                Obstacle => panic!("this should never happen"),
            },
            None => break,
        }
        current = loop {
            let next = current + idx_offset(&dir);
            if next == obstacle_location {
                dir = dir.right();
                continue;
            }
            match floor.get(&next) {
                Some(Obstacle) => dir = dir.right(),
                _ => {
                    if path.contains(&(current, dir)) {
                        return true;
                    }
                    //println!("({:?}, {:?}), {:?}", current.row, current.col, dir);
                    path.push((current, dir));
                    break next;
                }
            }
        };
    }
    false
}

fn part2(content: &str) -> usize {
    use LabTile::*;
    let floor = Matrix::<LabTile>::try_from_str(content).expect("parsing into matrix failed");
    let start = floor.find(&LabTile::Guard).expect("start point not found");
    let mut path = Vec::new();
    let mut dir = Direction::Up;
    let mut current = start;
    let mut loop_obstructions = HashSet::new();
    loop {
        // first we update the state of the machine

        let tile = floor.get(&current);
        match tile {
            Some(tile) => match tile {
                Tile | Guard => {}
                Obstacle => panic!("this should never happen"),
            },
            None => break,
        }

        let mut next;
        loop {
            next = current + idx_offset(&dir);
            match floor.get(&next) {
                Some(Obstacle) => dir = dir.right(),
                Some(Tile) => {
                    // here we spawn a loop check with once to the right but only if this tile has
                    // not been visited before
                    // we can test for a loop by inserting a block as this would not alter the
                    if !path.iter().any(|(idx, _)| idx == &next)
                        && path_has_loop(&floor, current, dir.right())
                    {
                        loop_obstructions.insert(next);
                    }
                    break;
                }
                _ => break,
            }
        }
        path.push((current, dir));
        current = next;
    }
    loop_obstructions.len()
}

fn main() {
    let content = util::load_file(6, 1, false).expect("failed to load input text file");
    let (solution, _path) = part1(&content);
    dbg!(solution);
    let solution = part2(&content);
    dbg!(solution);
}
#[cfg(test)]
mod test {
    use super::*;
    use adventofcode2024::util;

    #[test]
    fn check_loop() {
        use Direction::*;
        let loops = vec![
            (6, 4, Up),
            (6, 6, Left),
            (7, 6, Down),
            (8, 2, Up),
            (8, 4, Up),
            (8, 7, Left),
        ];

        let content = util::load_file(6, 1, true).unwrap();
        let floor = Matrix::<LabTile>::try_from_str(&content).expect("parsing into matrix failed");
        for (row, col, dir) in loops {
            let start = MatrixIdx::new(row, col);
            assert!(path_has_loop(&floor, start, dir))
        }
    }
}
