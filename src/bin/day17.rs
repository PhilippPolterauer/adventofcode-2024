use adventofcode2024::util;
use regex::Regex;

#[derive(Debug)]
struct CPU {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
}
impl CPU {
    fn new(a: i64, b: i64, c: i64, ip: usize) -> Self {
        Self { a, b, c, ip }
    }
    fn combo(&self, operand: u8) -> i64 {
        match operand {
            0 | 1 | 2 | 3 => operand as i64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            a => panic!("unexpected combo op '{:?}'", a),
        }
    }
    fn operate(&mut self, instructions: &[u8]) -> Vec<i64> {
        let mut out = Vec::new();
        while let (Some(&operator), Some(&operand)) =
            (instructions.get(self.ip), instructions.get(self.ip + 1))
        {
            self.ip += 2;
            match operator {
                0 => self.a = self.a / (1 << self.combo(operand)),
                1 => self.b = self.b ^ operand as i64,
                2 => self.b = self.combo(operand) % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = operand as usize
                    }
                }
                4 => self.b = self.b ^ self.c,
                5 => {
                    out.push(self.combo(operand) % 8);
                }
                6 => self.b = self.a / (1 << self.combo(operand)),
                7 => self.c = self.a / (1 << self.combo(operand)),
                a => panic!("unexpected combo op '{:?}'", a),
            }
        }
        out
    }
}
fn parse_input(content: &str) -> (CPU, Vec<u8>) {
    let (a, b) = content.split_once("\n\n").unwrap();
    let re = Regex::new(
        r"Register A: (\d*)
Register B: (\d*)
Register C: (\d*)",
    )
    .unwrap();
    let m = re.captures(a).unwrap();
    let cpu = CPU::new(
        m.get(1).unwrap().as_str().parse().unwrap(),
        m.get(2).unwrap().as_str().parse().unwrap(),
        m.get(3).unwrap().as_str().parse().unwrap(),
        0,
    );

    let (_, instructions) = b.split_once(": ").unwrap();
    let instructions: Vec<u8> = instructions
        .split(",")
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    (cpu, instructions)
}
fn part1(content: &str) -> i64 {
    let (mut cpu, instructions) = parse_input(content);
    let mut solution = 0;
    let out = cpu.operate(&instructions);
    for o in out {
        solution = solution * 10 + o;
    }
    solution
}
fn part2(content: &str) -> i64 {
    let (mut cpu, instructions) = parse_input(content);
    let mut a = 0;
    for i in instructions.iter().rev() {
        a <<= 3;
        for j in 0..1024 {
            cpu.a = a + j;
            cpu.b = 0;
            cpu.ip = 0;
            cpu.c = 0;
            let out = cpu.operate(&instructions[0..instructions.len() - 2]);
            if out.len() == 1 && out[0] == *i as i64 {
                println!("{:?}: {:?}", i, j);
                a = a + j;
                break;
            }
        }
    }
    a
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

    const TEST1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(TEST1), 4635635210);
    }
    #[test]
    fn test_part1_2() {
        let mut cpu = CPU::new(0, 0, 9, 0);
        cpu.operate(&[2, 6]);
        assert_eq!(cpu.b, 1);

        let mut cpu = CPU::new(10, 0, 0, 0);
        let out = cpu.operate(&[5, 0, 5, 1, 5, 4]);
        assert_eq!(out, vec![0, 1, 2]);

        let mut cpu = CPU::new(2024, 0, 0, 0);
        let out = cpu.operate(&[0, 1, 5, 4, 3, 0]);
        assert_eq!(out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(cpu.a, 0);

        let mut cpu = CPU::new(0, 29, 0, 0);
        let out = cpu.operate(&[1, 7]);
        assert_eq!(cpu.b, 26);

        let mut cpu = CPU::new(0, 2024, 43690, 0);
        let out = cpu.operate(&[4, 0]);
        assert_eq!(cpu.b, 44354);
    }
    #[test]
    fn test_part2_1() {}
    #[test]
    fn test_part2_2() {}
}
