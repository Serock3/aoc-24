use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let iter = input.split("\n\n");
    let keys = iter
        .clone()
        .filter(|chunk| chunk.starts_with("#####"))
        .map(|chunk| col_counts(chunk.lines().skip(1)))
        .collect_vec();
    let locks = iter
        .clone()
        .filter(|chunk| chunk.starts_with("....."))
        .map(|chunk| col_counts(chunk.lines().rev().skip(1)))
        .collect_vec();
    Some(
        keys.into_iter()
            .map(|key| {
                locks
                    .iter()
                    .filter(|lock| key.iter().zip_eq(*lock).all(|(a, b)| *a + *b <= 5))
                    .count()
            })
            .sum(),
    )
}

fn col_counts<'a, I: Iterator<Item = &'a str> + Clone + 'a>(lines: I) -> Vec<usize> {
    (0..5)
        .map(|col| {
            lines
                .clone()
                .take_while(|line| line.chars().nth(col).unwrap() == '#')
                .count()
        })
        .collect_vec()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
