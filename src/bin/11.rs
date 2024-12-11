use cached::proc_macro::cached;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let start = 25;
    Some(input.map(|x| get_number_of_stones(x, start)).sum())
}

#[cached]
fn get_number_of_stones(x: u64, start: i32) -> u64 {
    if start == 0 {
        return 1;
    }
    if x == 0 {
        return get_number_of_stones(1, start - 1);
    }
    let num_digits = x.ilog10() + 1;
    if num_digits % 2 == 0 {
        let left_split = x / 10u64.pow(num_digits / 2);
        let right_split = x % 10u64.pow(num_digits / 2);
        return get_number_of_stones(left_split, start - 1)
            + get_number_of_stones(right_split, start - 1);
    } else {
        get_number_of_stones(x * 2024, start - 1)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let start = 75;
    Some(input.map(|x| get_number_of_stones(x, start)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("125 17");
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
