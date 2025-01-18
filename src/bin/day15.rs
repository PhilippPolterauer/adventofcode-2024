use std::{
    collections::{HashSet, VecDeque},
    io::stdin,
};

use adventofcode2024::{
    matrix::{Direction, FromChar, Matrix, MatrixIdx, MatrixIdxOffset},
    util,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tiles2 {
    Wall,
    Free,
    BoxLeft,
    BoxRight,
    Robot,
}
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
fn expand(grid: &Matrix<Tiles>) -> Matrix<Tiles2> {
    let data = vec![Tiles2::Robot; grid.width() * grid.height() * 2];
    let mut grid2 = Matrix::<Tiles2>::from(data, grid.width() * 2);
    for (idx, tile) in grid.idx_value_iter() {
        let lidx = MatrixIdx::new(idx.row, idx.col * 2);
        let ridx = MatrixIdx::new(idx.row, idx.col * 2 + 1);
        use Tiles::*;
        (grid2[lidx], grid2[ridx]) = match tile {
            Free => (Tiles2::Free, Tiles2::Free),
            Box => (Tiles2::BoxLeft, Tiles2::BoxRight),
            Wall => (Tiles2::Wall, Tiles2::Wall),
            Robot => (Tiles2::Robot, Tiles2::Free),
        };
        //data.push(
    }
    grid2
}
fn show(grid: &Matrix<Tiles2>) {
    use Tiles2::*;
    for row in grid.rows() {
        for t in row {
            let c = match t {
                Free => '.',
                BoxLeft => '[',
                BoxRight => ']',
                Wall => '#',
                Robot => '@',
            };
            print!("{:}", c);
        }
        println!("");
    }
}
fn domove2(grid: &mut Matrix<Tiles2>, idx: MatrixIdx, direction: Direction) -> bool {
    use Tiles2::*;
    let start = grid[&idx];
    let target_idx = idx + offset(&direction);

    let right = offset(&Direction::Right);
    let left = offset(&Direction::Left);
    let start_right = grid.get(&(idx + right)).map(|x| *x);
    let start_left = grid.get(&(idx + left)).map(|x| *x);

    if let Some(target) = grid.get(&target_idx) {
        match target {
            Free => {
                grid[idx] = Free;
                grid[target_idx] = start;
                true
            }
            BoxLeft => {
                let ret = domove2(grid, target_idx, direction)
                    && domove2(grid, target_idx + right, direction);
                if ret {
                    grid[idx] = grid[target_idx];
                    grid[target_idx] = start;
                    grid[idx + right] = grid[target_idx + right];
                    grid[target_idx + right] = start_right.unwrap();
                }
                ret
            }
            BoxRight => {
                let ret = domove2(grid, target_idx, direction)
                    && domove2(grid, target_idx + left, direction);
                if ret {
                    grid[idx] = grid[target_idx];
                    grid[target_idx] = start;
                    grid[idx + left] = grid[target_idx + left];
                    grid[target_idx + left] = start_left.unwrap();
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

    for b in grid.find_all(&Tiles::Box) {
        solution += b.row * 100 + b.col
    }
    solution
}

fn can_move(
    grid: &Matrix<Tiles2>,
    idx: MatrixIdx,
    direction: Direction,
) -> Option<HashSet<MatrixIdx>> {
    use Direction::*;
    use Tiles2::*;
    let dir = offset(&direction);
    let left = offset(&Left);
    let right = offset(&Right);

    let mut visited = HashSet::new();
    let mut front = VecDeque::from([idx]);

    while let Some(idx) = front.pop_front() {
        let next = idx + dir;
        let tile = grid.get(&next).unwrap();
        match (tile, direction) {
            (Free, _) => (),
            (BoxRight | BoxLeft, Left | Right) => front.push_back(next),
            (BoxLeft | BoxRight, Up | Down) => {
                let off = if tile == &BoxLeft { right } else { left };
                let no = next + off;
                if !front.contains(&no) {
                    front.push_back(no);
                }
                if !front.contains(&next) {
                    front.push_back(next);
                }
            }
            (Wall, _) => return None,
            (Robot, _) => panic!("hit robot while moving, only one should exist"),
        }

        visited.insert(idx);
    }
    Some(visited)
}
fn perform_move(grid: &mut Matrix<Tiles2>, tomove: HashSet<MatrixIdx>, direction: Direction) {
    let dir = offset(&direction);
    let vals: Vec<_> = tomove.iter().map(|idx| (idx + &dir, grid[idx])).collect();
    for idx in tomove {
        grid[idx] = Tiles2::Free;
    }
    for (idx, val) in vals {
        grid[idx] = val;
    }
}

fn part2(content: &str) -> usize {
    let mut solution = 0;
    let (grid, directions) = parse_input(content);
    let mut grid2 = expand(&grid);
    let mut robot = grid2.find(&Tiles2::Robot).unwrap();
    for dir in directions {
        //show(&grid2);
        if let Some(front) = can_move(&grid2, robot, dir) {
            perform_move(&mut grid2, front, dir);
            robot = robot + offset(&dir)
        }
        //let mut s = String::new();
        //let _ = stdin().read_line(&mut s);
    }

    for b in grid2.find_all(&Tiles2::BoxLeft) {
        solution += b.row * 100 + b.col
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

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {}
}
