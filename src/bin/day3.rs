use std::{cmp::min_by_key, marker, ops::Mul};

use adventofcode2024::util;
use regex::{Match, Regex, RegexSet};

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
    return solution;
}
// write a tokenizer where i can decleratively declare tokens the can appear within a text

enum Tokens {
    Do,
    Dont,
    Mul(i32, i32),
    End,
}

// think about how we want to use  
pub trait IsToken<'a>{
    type MatchType;
    fn find_at(&self, haystack: &'a str, start: usize) -> Self::MatchType;

}

// struct RegexToken{
//     regex: Regex,
//     // _owns_t: marker::PhantomData<T>
// }
// impl <'a, T> RegexToken {
//     fn new(regex: &str)-> Self{        
//         Self { regex: Regex::new(regex).unwrap() }       
//     }
//     fn find_at(&self, haystack: &'a str, start: usize) -> Option<Match<'a>> {
//         self.regex.find_at(haystack, start)
//     }
// }
// struct DoToken (RegexToken);


// impl <'a> IsToken<'a> for RegexToken{
//     type MatchType = T;
//     fn find_at(&self, haystack: &'a str, start: usize) -> T {
//         self.regex.find_at(haystack, start)
//     }
// }



// fn find(haystack: &str, start: usize, regexes: &[Regexes]) -> (usize, Matches) {
//     if let Some(min) = regexes
//         .iter()
//         .filter_map(|regex| match &regex {
//             Regexes::Do(reg) | Regexes::Dont(reg) | Regexes::Mul(reg) => {
//                 let mat = reg.find_at(haystack, start);
//                 match regex { 
//                 }
//             }
//         })
//         .min_by_key(|mat| mat.start())
//     {
//         return min.start(), 
//     }

//     return (0, Matches::End);
// }

// fn part2() -> i32 {
//     let mulre = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";
//     let dore = r"do\(\)";
//     let dontre = r"don't\(\)";
//     let regexes = [
//         Regexes::Do(Regex::new(dore).unwrap()),
//         Regexes::Dont(Regex::new(dontre).unwrap()),
//         Regexes::Mul(Regex::new(mulre).unwrap()),
//     ];

//     let file = util::load_file(3, 1, false);
//     let mut solution = 0;

//     let mut start = 0;
//     let mut state = MulState::Do;
//     if let Ok(file) = file {
//         loop {
//             let (idx, mat) = find(&file, start);
//             state = match (&state, &mat) {
//                 (_, Matches::End) => break,
//                 (MulState::Do, Matches::Dont) => MulState::Dont,
//                 (MulState::Dont, Matches::Do) => MulState::Do,
//                 (MulState::Do, Matches::Mul(a, b)) => {
//                     solution += a * b;
//                     MulState::Do
//                 }
//                 _ => state,
//             };
//             start = idx;
//         }
//     }
//     return solution;
// }

fn main() {
    let solution = part1();
    dbg!(solution);
    // let solution = part2();
    dbg!(solution);
}
