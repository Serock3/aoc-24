use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|chunk| {
                let iter = chunk
                    .split(['+', ',', '=', '\n'])
                    .skip(1)
                    .step_by(2)
                    .map(|num| {
                        num.parse::<u32>()
                            .inspect_err(|_| println!("asdas: asdas: {num}"))
                            .unwrap()
                    });
                let (ax, ay, bx, by, px, py) = iter.collect_tuple().unwrap();
                for a in 0..=100 {
                    for b in 0..=100 {
                        if ax * a + bx * b == px && ay * a + by * b == py {
                            return a + b;
                        }
                    }
                }
                0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, None);
    }
}
