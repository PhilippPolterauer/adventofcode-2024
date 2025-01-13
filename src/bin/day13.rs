use std::{
    ops::{Add, Mul},
    usize,
};

use adventofcode2024::util;

#[derive(Debug, PartialEq)]
struct Vec2 {
    x: usize,
    y: usize,
}
#[derive(Debug, PartialEq)]
struct Equation {
    a: Vec2,
    b: Vec2,
    price: Vec2,
}
impl Equation {}

impl Mul<usize> for &Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: usize) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn part1(content: &str) -> usize {
    let mut solution = 0;
    let re = regex::Regex::new(
        r#"Button A: X\+(\d*), Y\+(\d*)
Button B: X\+(\d*), Y\+(\d*)
Prize: X=(\d*), Y=(\d*)"#,
    )
    .unwrap();
    let mut equations = Vec::new();
    for sp in content.split("\n\n") {
        for cap in re.captures_iter(sp) {
            let a = Vec2 {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            };
            let b = Vec2 {
                x: cap[3].parse().unwrap(),
                y: cap[4].parse().unwrap(),
            };
            let price = Vec2 {
                x: cap[5].parse().unwrap(),
                y: cap[6].parse().unwrap(),
            };

            let eq = Equation { a, b, price };
            equations.push(eq);
        }
    }
    for eq in equations {
        'outer: for b in 0..100 {
            for a in 0..100 {
                if &eq.b * b + &eq.a * a == eq.price {
                    println!("found solution for {:?} with ({:?},{:?})", eq, a, b);
                    solution += a * 3 + b;
                    break 'outer;
                }
            }
        }
    }
    solution
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
    #[test]
    fn test_mem() {}
}
