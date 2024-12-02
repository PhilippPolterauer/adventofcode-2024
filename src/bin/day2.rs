use core::time;
use std::{collections::HashMap, time::Instant};

use adventofcode2024::util;

// how to import util module from crate root
fn is_safe(line: &[i32]) -> bool {
    if line.len() <= 1 {
        return true;
    }
    let mut a = line[0];
    let diff0 = (line[1] - a).signum();

    for b in line.iter().skip(1) {
        let diff = b - a;
        if diff.abs() > 3 || diff.abs() == 0 || diff.signum() != diff0 {
            return false;
        }
        a = *b;
    }
    return true;
}
fn is_safe2(line: &[i32], skip: Option<usize>) -> bool {
    let mut a: Option<i32> = None;
    let mut dir0: Option<i32> = None;
    let mut diff;
    for (idx, b) in line.iter().enumerate() {
        if skip == Some(idx) {
            // we skip in case skip is set
            continue;
        }

        if let Some(a) = a {
            diff = b - a;
        } else {
            a = Some(*b);
            continue;
        }
        let dir = dir0.get_or_insert_with(|| diff.signum());

        if diff.abs() > 3 || diff.abs() == 0 || diff.signum() != *dir {
            if skip.is_some() {
                return false;
            }
            return is_safe2(line, Some(idx - 1))
                || is_safe2(line, Some(idx))
                || is_safe2(line, Some(0));
        }
        a = Some(*b);
    }
    return true;
}
fn part1() {
    let input = util::load_file(2, 1, false).unwrap();

    let mut solution = 0;
    for l in input.lines() {
        let line: Vec<i32> = l.split(" ").filter_map(|c| c.parse::<i32>().ok()).collect();
        if is_safe(&line) {
            solution += 1;
        }
    }

    dbg!(solution);
}

fn part2() {
    let input = util::load_file(2, 2, false).unwrap();

    let mut solution = 0;
    for l in input.lines() {
        let line: Vec<i32> = l.split(" ").filter_map(|c| c.parse::<i32>().ok()).collect();
        if is_safe2(&line, None) {
            solution += 1;
        }
    }

    dbg!(solution);
}
fn is_safe_corr(line: &[i32]) -> bool {
    for index in 0..line.len() {
        let mut nline = line.to_vec();
        nline.remove(index);

        if is_safe(&nline) {
            return true;
        }
    }
    return false;
}
fn part2corr() {
    let input = util::load_file(2, 2, false).unwrap();

    let mut solution = 0;
    for l in input.lines() {
        let line: Vec<i32> = l.split(" ").filter_map(|c| c.parse::<i32>().ok()).collect();
        if is_safe_corr(&line) {
            solution += 1;
        }
    }

    dbg!(solution);
}
fn main() {
    part1();
    let tcorr = Instant::now();
    part2corr();
    let dtcorr = Instant::now() - tcorr;
    dbg!(dtcorr);
    let tstart = Instant::now();
    part2();
    let dtfast = Instant::now() - tstart;
    dbg!(dtfast);
}
