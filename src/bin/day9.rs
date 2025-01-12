use std::usize;

use adventofcode2024::util;

/*

*/
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

#[derive(Clone, Copy, Debug)]
enum DiskObjects {
    Space(usize),
    File(usize, usize),
}

// Noice lets try to write some pseudocode for this problem
fn replace() {}

fn part2(content: &str) -> usize {
    let numbers: Vec<u32> = content
        .chars()
        .filter_map(|char| char.to_digit(10))
        .collect();

    let mut disk = Vec::<DiskObjects>::new();
    let mut id = 0;
    let mut space = false;
    for num in &numbers {
        let num = *num as usize;
        if space {
            disk.push(DiskObjects::Space(num));
        } else {
            disk.push(DiskObjects::File(num, id));
            id += 1;
        }
        space = !space;
    }
    disk.reverse();

    let mut current_id = id - 1;
    let mut idx = 0;
    loop {
        // first we find the idx of the diskobjects with current id

        let disk_object = disk[idx];
        //println!("looking for {:?} found {:?}", current_id, &disk_object);
        let action = match disk_object {
            DiskObjects::File(file_size, id) if (id == current_id) => {
                // we look for the last space that could fit the file
                let mut found = None;
                for i in idx..disk.len() {
                    //println!(
                    //    "index {:?}, checking {:?} current {:?}",
                    //    idx, i, &current_id
                    //);
                    match disk[i] {
                        DiskObjects::Space(space_size) if space_size >= file_size => {
                            found = Some((i, space_size, file_size));
                        }
                        _ => (),
                    }
                }
                found
            }
            _ => {
                idx += 1;
                continue;
            }
        };

        if let Some((space_idx, space_size, file_size)) = action {
            // here we need to perform the swapping operation
            // typically all objects before and after index are spaces so we combine them if they
            // are into one

            if space_size == file_size {
                //println!("spwapping {:?} with {:?}", &disk[idx], &disk[space_idx]);
                disk.swap(idx, space_idx);
            } else {
                if let Some(DiskObjects::Space(size)) = disk.get_mut(space_idx) {
                    *size = file_size;
                    //println!("{:?}", &disk[space_idx]);
                    disk.insert(space_idx, DiskObjects::Space(space_size - file_size));
                    //println!("after insert {:?}", &disk[space_idx]);
                    //println!(
                    //"spwapping 2nd {:?} with {:?}",
                    //    &disk[idx],
                    //    &disk[space_idx + 1]
                    //);
                    disk.swap(idx, space_idx + 1);
                } else {
                    panic!("we should not get here");
                }
            }
        }
        //println!("{:?}", disk.clone().reverse());
        if idx >= disk.len() || current_id == 0 {
            break;
        }
        current_id -= 1;
    }
    disk.reverse();
    //println!("{:?}", disk);

    let mut solution = 0;
    let mut idx = 0;
    for elem in disk {
        match elem {
            DiskObjects::Space(size) => idx += size,
            DiskObjects::File(space, id) => {
                for _ in 0..space {
                    //println!("{:?}, {:?}", idx, id);
                    solution += idx * id;
                    idx += 1;
                }
            }
        }
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
