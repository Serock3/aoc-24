use std::collections::btree_map::{self, BTreeMap};

use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let nums = input.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
    // dbg!(nums.clone().count());
    let mut position = 0;
    let fragments = nums
        .chunks(2)
        .enumerate()
        .map(|(id, slice)| {
            let fragment_size = slice[0] as usize;
            let free_space = slice[1] as usize;
            let fragment = (position, (id, fragment_size));
            position += fragment_size + free_space;
            fragment
        })
        .collect_vec();
    // let total_used_size = iter.clone().map(|(_, size)| size).sum::<usize>();
    let fragment_map = BTreeMap::<usize, (usize, usize)>::from_iter(fragments);
    let frag_iter = fragment_map
        .into_iter()
        .rev()
        .flat_map(|(_position, (id, fragment_size))| (0..fragment_size).map(move |s| (id, s)));
    for (id, pos) in frag_iter {

        fragment_map.
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("23331331214141314020");
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
