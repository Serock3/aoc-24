use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let left = input
        .lines()
        .map(|line| line.split_once("   ").unwrap().0.parse::<i32>().unwrap())
        .sorted();
    let right = input
        .lines()
        .map(|line| line.split_once("   ").unwrap().1.parse::<i32>().unwrap())
        .sorted();

    Some(left.zip(right).map(|(l, r)| (l - r).abs()).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let left = input
        .lines()
        .map(|line| line.split_once("   ").unwrap().0.parse::<i32>().unwrap())
        .sorted();
    let right = input
        .lines()
        .map(|line| line.split_once("   ").unwrap().1.parse::<i32>().unwrap())
        .sorted();

    Some(
        left.map(|l| l * right.clone().filter(|&r| l == r).count() as i32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
