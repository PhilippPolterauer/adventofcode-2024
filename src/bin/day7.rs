use adventofcode2024::util;

#[derive(Debug, PartialEq)]
struct Equation {
    lhs: usize,
    rhs: Vec<usize>,
}

enum Outcome {
    High,
    Solved,
    Low,
}

enum Operator {
    Add,
    Mul,
}
impl Operator {
    fn toggle(&self) -> Self {
        use Operator::*;
        match self {
            Add => Mul,
            Mul => Add,
        }
    }
}

impl Equation {
    fn parse(line: &str) -> Option<Self> {
        line.split_once(": ").and_then(|(lhs, rhs)| {
            if let Ok(lhs) = lhs.parse() {
                let rhs = rhs
                    .split(" ")
                    .filter_map(|numstr| numstr.parse().ok())
                    .collect();
                Some(Self { lhs, rhs })
            } else {
                None
            }
        })
    }
    fn max(&self) -> usize {
        let mut sum = self.rhs[0];
        for b in self.rhs.iter().skip(1) {
            if sum == 1 || b == &1 {
                sum += b;
            } else {
                sum *= b;
            }
        }
        sum
    }
    fn min(&self) -> usize {
        let mut sum = self.rhs[0];
        for b in self.rhs.iter().skip(1) {
            if sum == 1 || b == &1 {
                sum *= b;
            } else {
                sum += b;
            }
        }
        sum
    }
    fn solution_number(&self, num: u32) -> usize {
        let mut sum = self.rhs[0];
        for (idx, val) in self.rhs.iter().skip(1).enumerate() {
            if ((num >> idx) & 1) == 1 {
                //print!("+");
                sum += val
            } else {
                //print!("*");
                sum *= val
            }
        }

        //println!("");
        sum
    }
    fn solution_count(&self) -> u32 {
        let base: u32 = 2;

        base.pow((self.rhs.len() - 1) as u32)
    }
    fn check_solvable(&self) -> bool {
        for num in 0..self.solution_count() {
            if self.lhs == self.solution_number(num) {
                return true;
            }
        }
        false
    }
}
fn part1(content: &str) -> usize {
    let mut solution = 0;
    let equations = content
        .lines()
        .filter_map(Equation::parse)
        .collect::<Vec<Equation>>();
    for eq in equations {
        let base = 3u32;
        assert!(base.checked_pow((eq.rhs.len() - 1) as u32).is_some());
        if eq.check_solvable() {
            //println!("{:?} solved", eq);
            solution += eq.lhs;
        }
    }
    solution
}

fn compute_min_max(rhs: &[usize]) -> (usize, usize) {
    let mut min = rhs[0];
    let mut max = rhs[0];
    for b in rhs.iter().skip(1) {
        if b == &1 {
            max += 1;
        } else {
            if min == 1 {
                min = *b;
            }
            if max == 1 {
                max = b + 1;
            }
        }
    }
    (min, max)
}

fn is_separable(accumulate: usize, rhs: usize) -> Option<usize> {
    let base = 10usize;
    let module = base.pow(rhs.ilog10() + 1);
    let num = accumulate - rhs;
    let (lhs, ret) = (num / module, (num % module) == 0);
    //dbg!(&ret, &accumulate, &rhs, &lhs, &num, &module);
    ret.then_some(lhs)
}
fn is_factor(accumulate: usize, rhs: usize) -> Option<usize> {
    let (lhs, ret) = (accumulate / rhs, (accumulate % rhs) == 0);
    ret.then_some(lhs)
}

fn solve_recursive(accumulate: usize, rhss: &[usize]) -> bool {
    let rhs = rhss[0];
    if rhss.len() == 1 {
        return rhs == accumulate;
    }
    if accumulate < rhs {
        return false;
    }
    let rhss = &rhss[1..];
    is_separable(accumulate, rhs).is_some_and(|lhs| solve_recursive(lhs, rhss))
        || is_factor(accumulate, rhs).is_some_and(|lhs| solve_recursive(lhs, rhss))
        || solve_recursive(accumulate - rhs, rhss)
}

fn part2(content: &str) -> usize {
    let mut solution = 0;
    let equations = content
        .lines()
        .filter_map(Equation::parse)
        .collect::<Vec<Equation>>();
    for eq in equations {
        let mut rhss = eq.rhs;
        rhss.reverse();
        if solve_recursive(eq.lhs, &rhss) {
            solution += eq.lhs;
        }
    }
    solution
}

fn main() {
    let day_num = util::get_day();
    let content = util::load_file(day_num, 1, false).expect("failed to load input text file");

    let solution = part1(&content);
    dbg!(solution);
    let solution = part2(&content);
    dbg!(solution);
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Some(Equation {
                lhs: 10,
                rhs: vec![1, 2, 3, 4, 5, 6]
            }),
            Equation::parse("10: 1 2 3 4 5 6")
        )
    }
    #[test]
    fn test_max() {
        assert_eq!(
            Equation {
                lhs: 10,
                rhs: vec![1, 2, 3, 1, 5, 6]
            }
            .max(),
            ((1 + 2) * 3 + 1) * 5 * 6
        )
    }
    #[test]
    fn test_min() {
        assert_eq!(
            Equation {
                lhs: 10,
                rhs: vec![1, 2, 3, 1, 5, 6]
            }
            .min(),
            (2 + 3) + 5 + 6
        )
    }
}
