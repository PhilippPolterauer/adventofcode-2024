use std::{collections::HashMap, usize};

use adventofcode2024::util;

fn blink(stone: &str) -> Vec<String> {
    match stone {
        "" | "0" => vec!["1".to_string()],
        "1" => vec!["2024".to_string()],
        stone if stone.len() % 2 == 0 => {
            let (a, b) = stone.split_at(stone.len() / 2);
            vec![
                a.to_string(),
                match b.trim_start_matches("0") {
                    "" => "0".to_owned(),
                    num => num.to_owned(),
                },
            ]
        }
        stone => {
            vec![(stone.parse::<usize>().unwrap() * 2024).to_string()]
        }
    }
}
fn do_step(stones: &[String]) -> Vec<String> {
    let mut new_stones = Vec::new();
    for stone in stones {
        new_stones.extend(blink(stone));
    }
    new_stones
}

struct MemoizedStones {
    mem: HashMap<(usize, usize), usize>,
}

fn has_even_digits(num: usize) -> bool {
    let ndigits = num.ilog10() + 1;
    ndigits % 2 == 0
}
fn split(num: usize) -> (usize, usize) {
    let base = 10usize;
    let div = base.pow((num.ilog10() + 1) / 2);

    (num / div, num % div)
}
impl MemoizedStones {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }
    fn count_stones(&mut self, stone: usize, steps: usize) -> usize {
        //println!("{:?},{:?}", stone, steps);
        if steps == 0 {
            return 1;
        }

        if let Some(val) = self.mem.get(&(stone, steps)) {
            return *val;
        }

        let ret = match stone {
            0 => self.count_stones(1, steps - 1),
            1 => self.count_stones(2024, steps - 1),
            stone if has_even_digits(stone) => {
                let (a, b) = split(stone);
                self.count_stones(a, steps - 1) + self.count_stones(b, steps - 1)
            }
            stone => self.count_stones(stone * 2024, steps - 1),
        };
        self.mem.insert((stone, steps), ret);
        ret
    }
}
const STEPS: usize = 25usize;
fn part1(content: &str) -> usize {
    let mut stones: Vec<String> = content.split(' ').map(|s| s.trim().to_owned()).collect();

    for _ in 0..STEPS {
        stones = do_step(&stones);
    }

    stones.len()
}
fn part2(content: &str) -> usize {
    let stones: Vec<usize> = content
        .split(' ')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    let mut memcnt = MemoizedStones::new();
    let mut solution = 0;
    for stone in stones {
        solution += memcnt.count_stones(stone, 75);
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
    fn test_mem() {
        let stones = [125, 17];

        let mut mem = MemoizedStones::new();
        let sol = mem.count_stones(stones[0], 25) + mem.count_stones(stones[1], 25);

        assert_eq!(sol, 55312);
    }
    #[test]
    fn test_part1() {
        let content = "125 17";
        assert_eq!(part1(content), 55312);
    }
    #[test]
    fn test_split() {
        assert_eq!((100, 123), split(100123));
        assert_eq!((1233, 3123), split(12333123));
    }
}
