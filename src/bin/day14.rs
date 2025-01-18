use std::collections::HashSet;

use adventofcode2024::{
    matrix::{MatrixIdx, MatrixIdxOffset},
    util,
};
use regex::Regex;

fn parse_input(content: &str) -> (Vec<MatrixIdx>, Vec<MatrixIdxOffset>) {
    let re = Regex::new(r"p=(\d*),(\d*) v=([-]?\d*),([-]?\d*)").expect("creating regex failed!");
    let mut positions = Vec::new();
    let mut speeds = Vec::new();
    for cap in re.captures_iter(content) {
        let pos = MatrixIdx::new(
            cap.get(2).unwrap().as_str().parse().unwrap(),
            cap.get(1).unwrap().as_str().parse().unwrap(),
        );
        let speed = MatrixIdxOffset::new(
            cap.get(4).unwrap().as_str().parse().unwrap(),
            cap.get(3).unwrap().as_str().parse().unwrap(),
        );
        positions.push(pos);
        speeds.push(speed);
    }
    (positions, speeds)
}

fn wrap(pos: &mut MatrixIdx, nrows: usize, ncols: usize) {
    pos.row = pos.row % nrows;
    pos.col = pos.col % ncols;
}

fn step(poss: &mut [MatrixIdx], speeds: &[MatrixIdxOffset], nrows: usize, ncols: usize) {
    for (pos, speed) in poss.iter_mut().zip(speeds) {
        // we add nrows to avoid underflow, this assumes no offset is bigger then nrows or ncols
        pos.row += nrows;
        pos.col += ncols;
        *pos = *pos + speed;
        wrap(pos, nrows, ncols);
    }
}
fn safety_score(positions: &[MatrixIdx], nrows: usize, ncols: usize) -> i64 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for pos in positions {
        match pos {
            MatrixIdx { row, col } if *row > nrows / 2 && *col > ncols / 2 => q4 += 1,
            MatrixIdx { row, col } if *row > nrows / 2 && *col < ncols / 2 => q3 += 1,
            MatrixIdx { row, col } if *row < nrows / 2 && *col > ncols / 2 => q2 += 1,
            MatrixIdx { row, col } if *row < nrows / 2 && *col < ncols / 2 => q1 += 1,
            _ => (),
        }
    }

    q1 * q2 * q3 * q4
}

fn part1(content: &str, nrows: usize, ncols: usize) -> i64 {
    let (mut positions, speeds) = parse_input(content);
    for _ in 0..100 {
        step(&mut positions, &speeds, nrows, ncols);
    }

    safety_score(&positions, nrows, ncols)
}
fn show(positions: &[MatrixIdx], nrows: usize, ncols: usize) {
    for j in 0..nrows {
        for i in 0..ncols {
            //for (i, p) in positions {
            if positions.contains(&MatrixIdx::new(j, i)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn neighbours(pos: &MatrixIdx) -> HashSet<MatrixIdx> {
    HashSet::from(
        [(1, 0), (0, 1), (-1, 0), (0, -1)].map(|(row, col)| pos + &MatrixIdxOffset::new(row, col)),
    )
}
fn neighbour_score(positions: &HashSet<MatrixIdx>) -> usize {
    let mut score = 0;
    for pos in positions {
        score += positions.intersection(&neighbours(pos)).count();
    }
    score
}
fn part2(content: &str, nrows: usize, ncols: usize) -> i64 {
    use std::io::{stdin, stdout, Write};
    let (mut positions, speeds) = parse_input(content);
    let thresh = 400;
    let mut count = 0;
    loop {
        let pos_set: HashSet<_> = positions.iter().map(|x| *x).collect();
        //println!(
        //    "step: {:?}, score: {:?}",
        //    i,
        //    neighbour_score(&pos_set, nrows, ncols)
        //);
        let score = neighbour_score(&pos_set);
        if score > thresh {
            show(&positions, nrows, ncols);
            dbg!(&count);
            let _ = stdout().flush();
            let mut s = String::new();
            let input = stdin().read_line(&mut s);
            if let Ok(n) = input {
                println!("{:}, {:}", s, n);
                match s.as_str() {
                    "q\n" => break,
                    "s\n" => show(&positions, nrows, ncols),
                    _ => (),
                }
            }
        }
        step(&mut positions, &speeds, nrows, ncols);
        count += 1;
    }
    count
}

fn main() {
    let test = false;
    let (nrows, ncols) = if test { (7, 11) } else { (103, 101) };
    let content =
        util::load_file(util::get_day(), 1, test).expect("failed to load input text file");
    let solution = part1(&content, nrows, ncols);
    dbg!(solution);
    let solution = part2(&content, nrows, ncols);
    dbg!(solution);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {}
}
