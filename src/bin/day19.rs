use adventofcode2024::util;
#[derive(Debug, PartialEq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}
impl Color {
    fn from_char(c: &char) -> Self {
        match c {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => panic!("unexpected color {:?}", c),
        }
    }
}
type Towel = Vec<Color>;
type Design = Vec<Color>;
fn parse(content: &str) -> (Vec<Towel>, Vec<Design>) {
    let (top, bot) = content.split_once("\n\n").unwrap();
    (
        top.trim()
            .split(", ")
            .map(|towel_str| towel_str.chars().map(|c| Color::from_char(&c)).collect())
            .collect(),
        bot.lines()
            .map(|line| line.chars().map(|c| Color::from_char(&c)).collect())
            .collect(),
    )
}
fn is_possible(design: &[Color], towels: &[Vec<Color>]) -> bool {
    if design.is_empty() {
        return true;
    }
    for towel in towels {
        if design.starts_with(towel) && is_possible(&design[towel.len()..], towels) {
            return true;
        }
    }
    false
}
fn part1(content: &str) -> i64 {
    let mut solution = 0;
    let (towels, designs) = parse(content);
    for design in designs {
        if is_possible(&design, &towels) {
            solution += 1;
        }
    }
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

    use super::*;
    #[test]
    fn test_1() {
        let content = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(part1(content), 6)
    }
}
