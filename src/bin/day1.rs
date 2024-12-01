use std::collections::HashMap;

use adventofcode2024::util;

// how to import util module from crate root

// fn main() {
//     let input = util::load_file(1, 1, false).unwrap();
//     println!("Input: {}", input);

//     let mut left = Vec::new();
//     let mut right = Vec::new();

//     for l in input.lines() {
//         if let Some((a, b)) = l.split_once("   ") {
//             if let (Ok(aa), Ok(bb)) = (a.parse::<i32>(), b.parse::<i32>()) {
//                 left.push(aa);
//                 right.push(bb);
//             };
//         };
//     }

//     left.sort();
//     right.sort();

//     let mut solution = 0;
//     for (l, r) in left.iter().zip(right.iter()) {
//         solution += l.abs_diff(*r);
//     }
//     dbg!(solution);
// }

fn main() {
    let input = util::load_file(1, 2, false).unwrap();
    println!("Input: {}", input);

    let mut left = Vec::new();

    let mut count_map = HashMap::<i32, i32>::new();

    for l in input.lines() {
        if let Some((a, b)) = l.split_once("   ") {
            if let (Ok(aa), Ok(bb)) = (a.parse::<i32>(), b.parse::<i32>()) {
                left.push(aa);
                // get the entry for the right hand side and increase  the occurance count or insert 1
                count_map.entry(bb).and_modify(|val| *val += 1).or_insert(1);
            };
        };
    }
    let mut solution = 0;

    for l in left {
        solution += l * count_map.get(&l).unwrap_or(&0);
    }
    dbg!(solution);
}
