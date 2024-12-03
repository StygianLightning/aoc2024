use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Mul { a: u32, b: u32 },
    Do,
    Dont,
}

fn extract_instructions(s: &str) -> Vec<Instruction> {
    let re = Regex::new(r#"mul\((\d+),(\d+)\)|do\(\)|don't\(\)"#).unwrap();
    re.find_iter(s)
        .map(|m| {
            let match_str = m.as_str();
            match &match_str[0..4] {
                "mul(" => {
                    let mul_input = match_str.replace("mul(", "").replace(")", "");
                    let mut iter = mul_input.split(",");
                    let a = iter.next().unwrap().parse().unwrap();
                    let b = iter.next().unwrap().parse().unwrap();
                    Instruction::Mul { a, b }
                }
                "don'" => Instruction::Dont,
                "do()" => Instruction::Do,
                _ => {
                    unreachable!("unknown instruction: {match_str}")
                }
            }
        })
        .collect()
}

fn part1(instructions: &[Instruction]) -> u32 {
    let muls = instructions.iter().filter_map(|instr| {
        if let Instruction::Mul { a, b } = instr {
            Some((a, b))
        } else {
            None
        }
    });
    muls.fold(0, |acc, (a, b)| acc + a * b)
}

fn part2(instructions: &[Instruction]) -> u32 {
    instructions
        .iter()
        .fold((0, true), |(acc, enabled), instruction| match instruction {
            Instruction::Mul { a, b } => {
                if enabled {
                    (acc + a * b, enabled)
                } else {
                    (acc, enabled)
                }
            }
            Instruction::Do => (acc, true),
            Instruction::Dont => (acc, false),
        })
        .0
}

fn main() {
    let input = std::fs::read_to_string("input/day03.txt").unwrap();
    let instructions = extract_instructions(&input);
    let part1_res = part1(&instructions);
    println!("part 1: {part1_res}");
    let part2_res = part2(&instructions);
    println!("part 2: {part2_res}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_instructions() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let instructions = extract_instructions(input);
        assert_eq!(
            instructions,
            vec![
                Instruction::Mul { a: 2, b: 4 },
                Instruction::Dont,
                Instruction::Mul { a: 5, b: 5 },
                Instruction::Mul { a: 11, b: 8 },
                Instruction::Do,
                Instruction::Mul { a: 8, b: 5 }
            ]
        );
    }
}
