#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u32> for OpCode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct State {
    instruction_pointer: usize,
    // A, B, C
    registers: [u32; 3],
    data: Vec<u32>,
    output: Vec<u32>,
}

#[derive(Debug)]
struct Halted;

fn literal_value(operand: u32) -> u32 {
    operand
}

impl State {
    fn increase_instruction_pointer(&mut self) -> Result<(), Halted> {
        self.instruction_pointer += 2;
        if self.instruction_pointer >= self.data.len() {
            Err(Halted)
        } else {
            Ok(())
        }
    }

    fn combo_value(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand as usize - 4],
            _ => panic!("invalid literal: {operand}"),
        }
    }

    fn evaluate(&mut self) {
        while self.instruction_pointer < self.data.len() {
            if let Err(_) = self.step() {
                break;
            }
        }
    }

    fn step(&mut self) -> Result<(), Halted> {
        if self.instruction_pointer + 1 >= self.data.len() {
            return Err(Halted);
        }
        let op_code = OpCode::try_from(self.data[self.instruction_pointer]).map_err(|_| Halted)?;
        let operand = self.data[self.instruction_pointer + 1];
        match op_code {
            OpCode::Adv => {
                let num = self.registers[0];
                let denom = u32::pow(2, self.combo_value(operand));
                self.registers[0] = num / denom;
            }
            OpCode::Bxl => self.registers[1] = self.registers[1] ^ literal_value(operand),
            OpCode::Bst => self.registers[1] = self.combo_value(operand) % 8,
            OpCode::Jnz => {
                if self.registers[0] == 0 {
                    // do nothing
                } else {
                    self.instruction_pointer = literal_value(operand) as usize;
                    // do not increase instruction pointer
                    return Ok(());
                }
            }
            OpCode::Bxc => {
                self.registers[1] ^= self.registers[2];
            }
            OpCode::Out => {
                self.output.push(self.combo_value(operand) % 8);
            }
            OpCode::Bdv => {
                // same as Adv, including reading, but result is stored in B
                let num = self.registers[0];
                let denom = u32::pow(2, self.combo_value(operand));
                self.registers[1] = num / denom;
            }
            OpCode::Cdv => {
                // same as Adv, including reading, but result is stored in C
                let num = self.registers[0];
                let denom = u32::pow(2, self.combo_value(operand));
                self.registers[2] = num / denom;
            }
        }

        self.increase_instruction_pointer().map_err(|_| Halted)
    }
}

fn parse(input: &str) -> State {
    let mut it = input.lines();

    fn extract_value(text: &str) -> u32 {
        text.split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse()
            .unwrap()
    }
    let a = extract_value(it.next().unwrap());
    let b = extract_value(it.next().unwrap());
    let c = extract_value(it.next().unwrap());
    let data = it
        .skip(1)
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|d| d.parse().unwrap())
        .collect();

    State {
        instruction_pointer: 0,
        registers: [a, b, c],
        data,
        output: vec![],
    }
}

fn part1(state: &mut State) {
    state.evaluate();
}

fn main() {
    let input = std::fs::read_to_string("input/day17.txt").unwrap();
    let mut state = parse(&input);
    println!("{state:?}");

    part1(&mut state);

    println!("part 1 output:\n");
    for (i, x) in state.output.iter().enumerate() {
        print!("{x}");
        if i < state.output.len() - 1 {
            print!(",");
        }
    }

    println!();
}
