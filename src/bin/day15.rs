use std::mem::swap;

use adventofcode2024::{
    matrix::{Direction, FromChar, Matrix, MatrixIdx, MatrixIdxOffset},
    util,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tiles {
    Wall,
    Free,
    Box,
    Robot,
}

impl FromChar for Tiles {
    fn try_from_char(char: &char) -> Option<Self> {
        match char {
            '#' => Some(Tiles::Wall),
            '.' => Some(Tiles::Free),
            'O' => Some(Tiles::Box),
            '@' => Some(Tiles::Robot),
            _ => None,
        }
    }
}

fn parse_dir(content: &str) -> Vec<Direction> {
    content
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '\n' => None,
            _ => panic!("should not happen"),
        })
        .collect()
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
fn domove(grid: &mut Matrix<Tiles>, idx: MatrixIdx, direction: Direction) -> bool {
    use Tiles::*;
    let start = grid[&idx];
    let target_idx = idx + offset(&direction);

    if let Some(target) = grid.get(&target_idx) {
        match target {
            Free => {
                grid[idx] = Free;
                grid[target_idx] = start;
                true
            }
            Box => {
                let ret = domove(grid, target_idx, direction);
                if ret {
                    grid[idx] = grid[target_idx];
                    grid[target_idx] = start;
                }
                ret
            }
            Wall => false,
            Robot => panic!("found robot on target"),
        }
    } else {
        false
    }
}

fn parse_input(content: &str) -> (Matrix<Tiles>, Vec<Direction>) {
    let (a, b) = content.split_once("\n\n").unwrap();
    (parse_grid(a), parse_dir(b))
}
fn parse_grid(content: &str) -> Matrix<Tiles> {
    Matrix::<Tiles>::try_from_str(content).unwrap()
}
fn part1(content: &str) -> usize {
    let mut solution = 0;

    let (mut grid, directions) = parse_input(content);
    let mut robot = grid.find(&Tiles::Robot).unwrap();
    for dir in directions {
        if domove(&mut grid, robot, dir) {
            robot = robot + offset(&dir)
        }
    }
    
    for b in grid.find_all(&Tiles::Box){
        solution += b.row * 100 + b.col
    }
    solution
}
fn part2(content: &str) -> i64 {
    let solution = 0;
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

    #[test]
    fn test_1() {}
}
