use adventofcode2024::util;

fn part1(content: &str) -> usize {
    let numbers: Vec<u32> = content
        .chars()
        .filter_map(|char| char.to_digit(10))
        .collect();

    let mut disk = Vec::<Option<usize>>::new();
    let mut id = 0;
    let mut space = false;
    for num in &numbers {
        let num = *num as usize;
        if space {
            let mut vec = vec![None; num];
            disk.append(&mut vec);
        } else {
            let mut vec = vec![Some(id); num];
            disk.append(&mut vec);
            id += 1;
        }
        space = !space;
    }
    dbg!(&disk[disk.len() - 1]);
    dbg!(&numbers.len());
    let mut idx = 0;
    let mut rev_idx = disk.len();
    let mut solution = 0;
    loop {
        let id = if let Some(id) = disk[idx] {
            id
        } else {
            loop {
                rev_idx -= 1;
                if let Some(id) = disk[rev_idx] {
                    break id;
                }
            }
        };

        if rev_idx <= idx {
            break;
        }
        solution += idx * id;

        idx += 1;
    }

    solution
}
fn part2(content: &str) -> usize {
    0
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
mod test {}
