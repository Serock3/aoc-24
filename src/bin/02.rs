advent_of_code::solution!(2);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let levels = line
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().unwrap());
                is_safe(levels)
            })
            .filter(|x| *x)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| {
                let levels = line
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect_vec();
                (0..levels.len())
                    .map(|i| is_safe(levels[0..i].iter().chain(levels[i + 1..].iter()).copied()))
                    .any(|b| b)
            })
            .filter(|x| *x)
            .count(),
    )
}

fn is_safe(levels: impl Iterator<Item = i32>) -> bool {
    let diffs = levels.tuple_windows().map(|(a, b)| b - a).collect_vec();
    let min = *diffs.iter().min().unwrap();
    let max = *diffs.iter().max().unwrap();
    let too_low = min < -3;
    let too_high = max > 3;
    let is_zero = diffs.iter().contains(&0);
    let not_monotonic = min * max < 0;
    !(too_low || too_high || is_zero || not_monotonic)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
