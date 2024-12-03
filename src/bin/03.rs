use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|c| c.extract())
            .map(|(_cap, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    Some(
        re.captures_iter(input)
            .map(|cap| match &cap[0] {
                "do()" => {
                    enabled = true;
                    0
                }
                "don't()" => {
                    enabled = false;
                    0
                }
                _mul if enabled => cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap(),
                _ => 0,
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
