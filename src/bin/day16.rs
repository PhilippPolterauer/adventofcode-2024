use std::collections::{HashMap, VecDeque};

use adventofcode2024::{
    matrix::{Direction, FromChar, Matrix, MatrixIdx, MatrixIdxOffset, ALL_DIRECTIONS},
    util,
};
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tiles {
    Wall,
    Free,
    Start,
    End,
}
impl FromChar for Tiles {
    fn try_from_char(char: &char) -> Option<Self> {
        match char {
            '#' => Some(Tiles::Wall),
            '.' => Some(Tiles::Free),
            'S' => Some(Tiles::Start),
            'E' => Some(Tiles::End),
            _ => None,
        }
    }
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

enum Input {
    TurnLeft,
    TurnRight,
    Step,
}
const ALL_INPUTS: [Input; 3] = [Input::TurnLeft, Input::TurnRight, Input::Step];
#[derive(PartialEq, Hash, Eq, Clone, Copy, Debug)]
struct State(MatrixIdx, Direction);
fn backward(state: &State, input: Input) -> (State, usize) {
    let State(pos, dir) = state;
    match input {
        Input::TurnLeft => (State(*pos, dir.right()), 1000),
        Input::TurnRight => (State(*pos, dir.left()), 1000),
        Input::Step => (State(pos - offset(dir), *dir), 1),
    }
}

fn part1(content: &str) -> usize {
    let grid = Matrix::<Tiles>::try_from_str(content).unwrap();
    let solution = 0;
    let start = grid.find(&Tiles::Start).unwrap();
    let end = grid.find(&Tiles::End).unwrap();

    // find possible end states and insert them into costmap with 0
    let mut front: VecDeque<_> = ALL_DIRECTIONS
        .iter()
        .filter_map(|x| {
            grid.get(&(end + offset(x)))
                .and_then(|v| (v == &Tiles::Free).then_some((State(end, x.opposite()), 0)))
        })
        .collect();
    let mut cost_map = HashMap::new();
    for (k, v) in front.iter() {
        cost_map.insert(*k, *v);
    }

    while let Some((state, total_cost)) = front.pop_front() {
        for input in ALL_INPUTS {
            let (next_state, cost) = backward(&state, input);
            let new_total_cost = total_cost + cost;
            if let Some(Tiles::Free | Tiles::Start) = grid.get(&next_state.0) {
                cost_map
                    .entry(next_state)
                    .and_modify(|old_total_cost| {
                        if &new_total_cost < old_total_cost {
                            front.push_back((next_state, new_total_cost));
                            *old_total_cost = new_total_cost;
                        }
                    })
                    .or_insert_with(|| {
                        front.push_back((next_state, new_total_cost));
                        new_total_cost
                    });
            }
        }
    }
    cost_map[&State(start, Direction::Right)]
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
