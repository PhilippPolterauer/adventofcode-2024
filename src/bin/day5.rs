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
        true
    }
    /// fixes the input slice by swapping the two elements if the rule demands it
    fn fix(&self, input: &mut [i32]) -> bool {
        let mut found_after = None;
        let mut found_before = None;
        for (index, num) in input.iter().enumerate() {
            if num == &self.before {
                found_before = Some(index);
                break;
            }
            if num == &self.after {
                found_after = Some(index);
            }
        }
        if let (Some(before), Some(after)) = (found_before, found_after) {
            input.swap(before, after);
            return true;
        }
        false
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
                (!line.is_empty()).then_some(
                    line.split(",")
                        .filter_map(|page| page.parse::<i32>().ok())
                        .collect(),
                )
            })
            .collect(),
    )
}

fn part1(content: &str) -> i32 {
    let (rules, updates) = parse_input(content);
    let mut solution = 0;
    for page_list in updates {
        if rules.iter().all(|rule| rule.check(&page_list)) {
            solution += page_list[page_list.len() / 2];
        }
    }
    solution
}
fn part2(content: &str) -> i32 {
    let (rules, updates) = parse_input(content);

    let mut solution = 0;
    let mut to_fix = Vec::new();
    for page_list in updates {
        if rules.iter().any(|rule| !rule.check(&page_list)) {
            // here we try to fix the rule by iterating until all rules are satisfied
            to_fix.push(page_list);
        }
    }
    dbg!(&to_fix);
    for page_list in to_fix.iter_mut() {
        while rules.iter().any(|rule| rule.fix(page_list)) {}
        solution += page_list[page_list.len() / 2];
    }
    dbg!(to_fix);
    solution
}

fn main() {
    let content = util::load_file(5, 1, false).expect("failed to load input text file");
    let solution = part1(&content);
    dbg!(solution);

    let solution = part2(&content);
    dbg!(solution);
}
