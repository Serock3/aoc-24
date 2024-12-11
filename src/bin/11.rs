use cached::proc_macro::cached;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let num_blinks = 25;

    solve(input, num_blinks)
}

fn solve(input: &str, num_blinks: u32) -> Option<u64> {
    let input = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    Some(input.map(|x| get_number_of_stones(x, num_blinks)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let num_blinks = 75;

    solve(input, num_blinks)
}

#[cached]
fn get_number_of_stones(stone_num: u64, blink: u32) -> u64 {
    if blink == 0 {
        return 1;
    }
    if stone_num == 0 {
        return get_number_of_stones(1, blink - 1);
    }
    let num_digits = stone_num.ilog10() + 1;
    if num_digits % 2 == 0 {
        let left_split = stone_num / 10u64.pow(num_digits / 2);
        let right_split = stone_num % 10u64.pow(num_digits / 2);
        return get_number_of_stones(left_split, blink - 1)
            + get_number_of_stones(right_split, blink - 1);
    } else {
        get_number_of_stones(stone_num * 2024, blink - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("125 17");
        assert_eq!(result, Some(55312));
    }
}
