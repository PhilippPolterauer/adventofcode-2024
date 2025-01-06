use adventofcode2024::util;
#[derive(Debug)]
struct PageRule {
    before: i32,
    after: i32,
}
impl PageRule {
    fn new(before: i32, after: i32) -> Self {
        Self { before, after }
    }
    fn try_parse(input: &str) -> Option<Self> {
        input.split_once("|").and_then(|(a, b)| {
            if let (Ok(before), Ok(after)) = (a.parse::<i32>(), b.parse::<i32>()) {
                Some(PageRule::new(before, after))
            } else {
                None
            }
        })
    }
    fn check(&self, input: &[i32]) -> bool {
        let mut found_after = false;
        for num in input {
            if num == &self.before {
                return !found_after;
            }
            if num == &self.after {
                found_after = true;
            }
        }
        return true;
    }
}
fn parse_input(content: &str) -> (Vec<PageRule>, Vec<Vec<i32>>) {
    let (rules_str, updates_str) = content
        .split_once("\n\n")
        .expect("splitting into rules and updates failed!");

    (
        rules_str
            .split("\n")
            .filter_map(PageRule::try_parse)
            .collect(),
        updates_str
            .split("\n")
            .filter_map(|line| {
                (line != "").then_some(
                    line.split(",")
                        .filter_map(|page| page.parse::<i32>().ok())
                        .collect(),
                )
            })
            .collect(),
    )
}

fn part1(content: &str) -> i32 {
    let (rules, updates) = parse_input(&content);
    let mut solution = 0;
    for page_list in updates {
        if rules.iter().all(|rule| rule.check(&page_list)) {
            solution += page_list[page_list.len() / 2];
        }
    }
    solution
}
fn part2(content: &str) -> i32 {
    let solution = 0;
    solution
}

fn main() {
    let content = util::load_file(5, 1, false).expect("failed to load input text file");
    let solution = part1(&content);
    dbg!(solution);

    //let content = util::load_file(4, 1, false).expect("failed to load input text file");
    //let solution = part2(&content);
    //dbg!(solution);
}
