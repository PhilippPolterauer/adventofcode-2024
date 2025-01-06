use adventofcode2024::util;
use regex::Regex;

fn part1() -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let file = util::load_file(3, 1, false);
    let mut solution = 0;
    if let Ok(file) = file {
        for line in file.lines() {
            for name in re.captures_iter(line) {
                if let (Ok(a), Ok(b)) = (&name[1].parse::<i32>(), &name[2].parse::<i32>()) {
                    solution += a * b;
                }
            }
        }
    }
    solution
}
// write a tokenizer where i can decleratively declare tokens the can appear within a text
#[derive(Debug)]
enum Tokens {
    Do,
    Dont,
    Mul(i32, i32),
}

// think about how we want to use
pub trait IsToken<'a> {
    type MatchType;
    fn find_at(&self, haystack: &'a str, start: usize) -> Self::MatchType;
}

fn part2() -> i32 {
    let mulre = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let dore = Regex::new(r"do\(\)").unwrap();
    let dontre = Regex::new(r"don't\(\)").unwrap();

    let file = util::load_file(3, 1, false);
    let mut solution = 0;

    let mut tokens = Vec::new();
    if let Ok(content) = file {
        for m in dore.find_iter(&content) {
            tokens.push((m.start(), Tokens::Do));
        }

        for m in dontre.find_iter(&content) {
            tokens.push((m.start(), Tokens::Dont));
        }
        for m in mulre.find_iter(&content) {
            let cap = mulre.captures_at(&content, m.start());
            if let Some(cap) = cap {
                if let (Some(a), Some(b)) = (cap.get(1), cap.get(2)) {
                    if let (Ok(ra), Ok(rb)) = (a.as_str().parse::<i32>(), b.as_str().parse::<i32>())
                    {
                        tokens.push((m.start(), Tokens::Mul(ra, rb)));
                    }
                }
            }
        }
    }
    tokens.sort_by_key(|(s, _)| *s);
    let mut enabled = true;
    for (_, token) in tokens {
        let (next, change) = match (enabled, token) {
            (true, Tokens::Mul(a, b)) => (true, a * b),
            (true, Tokens::Dont) => (false, 0),
            (false, Tokens::Do) => (true, 0),
            _ => (enabled, 0),
        };
        solution += change;
        enabled = next;
    }
    solution
}

fn main() {
    let solution = part1();
    dbg!(solution);
    let solution = part2();
    dbg!(solution);
}
