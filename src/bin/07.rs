use itertools::{repeat_n, Itertools};
use strum_macros::EnumIter;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    solve::<OperatorsPart1>(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve::<OperatorsPart2>(input)
}

fn solve<O: Operator>(input: &str) -> Option<u64> {
    let equations = parse_equations(input);

    Some(
        equations
            .filter(is_solvable::<O>)
            .map(|Equation { value, .. }| value)
            .sum(),
    )
}

struct Equation {
    value: u64,
    numbers: Vec<u32>,
}

fn parse_equations(input: &str) -> impl Iterator<Item = Equation> + '_ {
    input.lines().map(|line| {
        let (head, tail) = line.split_once(':').unwrap();
        let value = head.parse::<u64>().unwrap();
        let numbers = tail
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<u32>, _>>()
            .unwrap();
        Equation { value, numbers }
    })
}

trait Operator: strum::IntoEnumIterator + Copy {
    fn operate(&self, result: u64, number: u32) -> Option<u64>;
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum OperatorsPart1 {
    Add,
    Multiply,
}
impl Operator for OperatorsPart1 {
    fn operate(&self, result: u64, number: u32) -> Option<u64> {
        match self {
            OperatorsPart1::Add => result.checked_add(number as u64),
            OperatorsPart1::Multiply => result.checked_mul(number as u64),
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum OperatorsPart2 {
    Add,
    Multiply,
    Concatenate,
}
impl Operator for OperatorsPart2 {
    fn operate(&self, result: u64, number: u32) -> Option<u64> {
        fn concatenate(mut result: u64, number: u32) -> Option<u64> {
            let num_digits = number.ilog10() + 1;
            result = result.checked_mul(10u64.pow(num_digits))?;
            result = result.checked_add(number as u64)?;
            Some(result)
        }
        match self {
            OperatorsPart2::Add => result.checked_add(number as u64),
            OperatorsPart2::Multiply => result.checked_mul(number as u64),
            OperatorsPart2::Concatenate => concatenate(result, number),
        }
    }
}

fn is_solvable<O: Operator>(equation: &Equation) -> bool {
    let Equation { value, numbers } = equation;
    repeat_n(O::iter(), numbers.len() - 1)
        .multi_cartesian_product()
        .any(|ops| {
            let mut result = numbers[0] as u64;
            for (op, &number) in ops.iter().zip(&numbers[1..]) {
                let Some(acc) = op.operate(result, number) else {
                    return false;
                };
                result = acc;
            }
            result == *value
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
