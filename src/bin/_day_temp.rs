use adventofcode2024::util;
fn part1(content: &str) -> i64 {
    let solution = 0;
    solution
}
fn part2(content: &str) -> i64 {
    let solution = 0;
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

    #[test]
    fn test_1() {}
}
