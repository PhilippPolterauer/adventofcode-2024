use std::{
    i64,
    ops::{Add, Mul},
};

use adventofcode2024::util;

#[derive(Debug, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}
#[derive(Debug, PartialEq)]
struct Equation {
    a: Vec2,
    b: Vec2,
    c: Vec2,
}
impl Equation {}

impl Mul<i64> for &Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: i64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul<i64> for &Equation {
    type Output = Equation;
    fn mul(self, rhs: i64) -> Self::Output {
        Equation {
            a: &self.a * rhs,
            b: &self.b * rhs,
            c: &self.c * rhs,
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

fn parse_equations(content: &str) -> Vec<Equation> {
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
            let c = Vec2 {
                x: cap[5].parse().unwrap(),
                y: cap[6].parse().unwrap(),
            };

            let eq = Equation { a, b, c };
            equations.push(eq);
        }
    }
    equations
}
fn part1(content: &str) -> i64 {
    let mut solution = 0;
    let equations = parse_equations(content);
    for eq in equations {
        'outer: for b in 0..100 {
            for a in 0..100 {
                if &eq.b * b + &eq.a * a == eq.c {
                    //println!("found solution for {:?} with ({:?},{:?})", eq, a, b);
                    solution += a * 3 + b;
                    break 'outer;
                }
            }
        }
    }
    solution
}
pub fn gcd(mut n: i64, mut m: i64) -> i64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
const ADDIT: i64 = 10000000000000;
fn part2(content: &str) -> i64 {
    let mut solution = 0;
    let equations = parse_equations(content);

    for mut eq in equations {
        eq.c.x += ADDIT;
        eq.c.y += ADDIT;

        if eq.a.x * eq.b.y == eq.a.y * eq.b.x {
            println!("no solution");
        } else {
            let detx = eq.c.x * eq.b.y - eq.c.y * eq.b.x;
            let dety = eq.a.x * eq.c.y - eq.a.y * eq.c.x;
            let det = eq.a.x * eq.b.y - eq.a.y * eq.b.x;
            if detx % det == 0 && dety % det == 0 {
                let a = detx / det;
                let b = dety / det;
                if a < 0 || b < 0 {
                    println!("error");
                }
                solution += a * 3 + b;
            }
        }
        dbg!(eq);
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
    use super::*;
    #[test]
    fn test_mem() {}
}
