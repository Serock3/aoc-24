use hashbrown::HashMap;
use itertools::Itertools;

advent_of_code::solution!(19);
// 3502960 too low

pub fn part_one(input: &str) -> Option<usize> {
    let (head, tail) = input.split_once("\n\n").unwrap();
    let towels = head.split(", ").collect_vec();
    let designs = tail.lines();
    Some(
        designs
            .filter(|design| is_composable(design, &towels))
            .count(),
    )
}

fn is_composable(design: &str, towels: &[&str]) -> bool {
    for towel in towels {
        if design.starts_with(towel) && is_composable(&design[towel.len()..], towels) {
            return true;
        }
    }
    design.is_empty()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (head, tail) = input.split_once("\n\n").unwrap();
    let towels = head.split(", ").collect_vec();
    let designs = tail.lines();
    Some(
        designs
            .map(|design| {
                let mut cache = HashMap::new();
                count_compositions(design, &towels, &mut cache)
            })
            .sum(),
    )
}

fn count_compositions<'a>(
    design: &'a str,
    towels: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        1
    } else {
        if let Some(&count) = cache.get(design) {
            return count;
        }
        let res = towels
            .iter()
            .filter(|&&towel| design.starts_with(towel))
            .map(|towel| count_compositions(&design[towel.len()..], towels, cache))
            .sum();
        cache.insert(design, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        );
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        );
        assert_eq!(result, Some(16));
    }
}
