use itertools::Itertools;

advent_of_code::solution!(17);

const REGISTER_LEN: usize = "Register _: ".len();
const PROGRAM_LEN: usize = "Program: ".len();

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        use Instruction::*;
        match value {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => panic!(),
        }
    }
}

impl Instruction {
    fn process(&self, computer: &mut Computer, operand: u8) -> bool {
        use Instruction::*;
        let literal = operand as u64;
        let combo = || match operand {
            0..=3 => operand as u64,
            4 => computer.register_a,
            5 => computer.register_b,
            6 => computer.register_c,
            _ => panic!(),
        };
        match self {
            Adv => computer.register_a /= 2u64.pow(combo() as u32),
            Bxl => computer.register_b ^= literal,
            Bst => computer.register_b = combo() & 0b111,
            Jnz => {
                if computer.register_a == 0 {
                    return true;
                }
                computer.instruction_pointer = literal as u8;
                return false;
            },
            Bxc => computer.register_b ^= computer.register_c,
            Out => computer.output.push((combo() & 0b111) as u8),
            Bdv => computer.register_b = computer.register_a / 2u64.pow(combo() as u32),
            Cdv => computer.register_c = computer.register_a / 2u64.pow(combo() as u32),
        }
        true
    }
}

#[derive(Debug, Default)]
struct Computer {
    program: Vec<u8>,
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: u8,
    output: Vec<u8>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let register_a = lines.next().and_then(|line| line[REGISTER_LEN..].parse::<u64>().ok()).unwrap_or_default();
        let register_b = lines.next().and_then(|line| line[REGISTER_LEN..].parse::<u64>().ok()).unwrap_or_default();
        let register_c = lines.next().and_then(|line| line[REGISTER_LEN..].parse::<u64>().ok()).unwrap_or_default();
        lines.next();
        let program = lines.next().iter()
            .flat_map(|line| line[PROGRAM_LEN..].split(','))
            .flat_map(|number| number.parse::<u8>().ok())
            .collect_vec();
        Computer {
            program,
            register_a,
            register_b,
            register_c,
            ..Computer::default()
        }
    }

    fn execute(&mut self) {
        while self.instruction_pointer < self.program.len() as u8 {
            let pointer = self.instruction_pointer as usize;
            let (opcode, operand) = (self.program[pointer], self.program[pointer + 1]);
            let instruction = Instruction::from(opcode);
            if instruction.process(self, operand) {
                self.instruction_pointer += 2;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::parse(input);
    computer.execute();
    Some(computer.output.into_iter().join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut computer = Computer::parse(input); 
    let mut register_a_candidate = 1;
    loop {
        computer.register_a = register_a_candidate;
        computer.register_b = 0;
        computer.register_b = 0;
        computer.instruction_pointer = 0;
        computer.output.clear();
        computer.execute();
        if computer.program.ends_with(&computer.output) {
            if computer.output.len() == computer.program.len() {
                break;
            }
            register_a_candidate <<= 3;
            continue
        }
        while register_a_candidate & 0b111 == 0b111 {
            register_a_candidate >>= 3;
        }
        register_a_candidate += 1;
    };
    Some(register_a_candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(117440));
    }
}
