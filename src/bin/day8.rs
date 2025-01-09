use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use adventofcode2024::{
    matrix::{FromChar, Matrix, MatrixElement, MatrixIdx},
    util,
};

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
struct Char(char);

impl FromChar for Char {
    fn try_from_char(char: &char) -> Option<Self> {
        Some(Char(*char))
    }
}
impl MatrixElement for Char {}

fn part1(content: &str) -> usize {
    let grid = Matrix::<Char>::try_from_str(content).unwrap();
    let mut antiodes = HashSet::new();
    let mut antennas = HashMap::<Char, Vec<MatrixIdx>>::new();
    for (idx, elem) in grid.idx_value_iter() {
        if elem == &Char('.') {
            continue;
        }
        let entry = antennas.entry(*elem).or_insert(Vec::new());

        for other in entry.iter() {
            let delta = other - idx;
            let a = other + &delta;
            let b = idx - delta;

            if grid.is_valid_idx(&a) {
                antiodes.insert(a);
            }
            if grid.is_valid_idx(&b) {
                antiodes.insert(b);
            }
        }
        entry.push(idx);
    }

    antiodes.len()
}
fn part2(content: &str) -> usize {
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
    use adventofcode2024::util;

    #[test]
    fn test_parse() {}
}
