use itertools::Itertools;

advent_of_code::solution!(17);
// 503576154

type Reg = u128;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Regs {
    a: Reg,
    b: Reg,
    c: Reg,
}

pub fn part_one(input: &str) -> Option<Reg> {
    let (program, mut regs) = parse_input(input);
    let output = run(program, &mut regs);
    let join = dbg!(output.into_iter().join(""));
    join.parse::<Reg>().ok()
}

fn parse_input(input: &str) -> (Vec<u8>, Regs) {
    let (regs_str, program_str) = input.split_once("\n\n").unwrap();
    let (reg_a, reg_b, reg_c) = regs_str
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<Reg>().unwrap())
        .collect_tuple()
        .unwrap();
    let program = program_str
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();

    let regs = Regs {
        a: reg_a,
        b: reg_b,
        c: reg_c,
    };
    (program, regs)
}

fn run(program: Vec<u8>, regs: &mut Regs) -> Vec<Reg> {
    let mut output = vec![];
    let mut instruction_pointer = 0;
    while let Some(chunk) = program.get(instruction_pointer..instruction_pointer + 2) {
        let opcode = chunk[0];
        let operand = chunk[1];
        execute_instruction(regs, &mut output, &mut instruction_pointer, opcode, operand);
    }
    output
}

fn execute_instruction(
    regs: &mut Regs,
    output: &mut Vec<u128>,
    instruction_pointer: &mut usize,
    opcode: u8,
    operand: u8,
) {
    match opcode {
        // adv
        0 => regs.a >>= combo(operand, regs),
        // bxl
        1 => regs.b = Reg::from((regs.b as u8) ^ operand),
        // bst
        2 => regs.b = (combo(operand, regs) as u8 % 8).into(),
        // jnz
        3 => {
            if regs.a != 0 {
                *instruction_pointer = operand as usize;
                return;
            }
        }
        // bxc
        4 => regs.b ^= regs.c,
        // out
        5 => output.push(combo(operand, regs) % 8),
        // bdv
        6 => regs.b = regs.a >> combo(operand, regs),
        // cdv
        7 => regs.c = regs.a >> combo(operand, regs),
        _ => panic!("Invalid opcode"),
    }
    *instruction_pointer += 2;
}

fn combo(op: u8, regs: &Regs) -> Reg {
    match op {
        n @ 0..=3 => n.into(),
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => panic!("Invalid combo"),
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_example_one() {
        let input = "Register A: 0
Register B: 0
Register C: 9

Program: 2,6";
        let (program, mut regs) = parse_input(input);
        let output = run(program, &mut regs);
        assert_eq!(regs.b, 1);
        assert!(output.is_empty());
    }

    #[test]
    fn test_example_two() {
        let input = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";
        let (program, mut regs) = parse_input(input);
        let output = run(program, &mut regs);
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn test_example_three() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let (program, mut regs) = parse_input(input);
        let output = run(program, &mut regs);
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(regs.a, 0);
    }

    #[test]
    fn test_example_four() {
        let input = "Register A: 0
Register B: 29
Register C: 0

Program: 1,7";
        let (program, mut regs) = parse_input(input);
        let _output = run(program, &mut regs);
        assert_eq!(regs.b, 26);
    }

    #[test]
    fn test_example_five() {
        let input = "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0";
        let (program, mut regs) = parse_input(input);
        let _output = run(program, &mut regs);
        assert_eq!(regs.b, 44354);
    }
    #[test]
    fn test_part_one() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let (program, mut regs) = parse_input(input);
        let output = run(program, &mut regs);
        assert_eq!(output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
