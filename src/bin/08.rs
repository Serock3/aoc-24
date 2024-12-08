// use std::collections::HashMap;

use advent_of_code::Pos;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let antennas = get_antenna_map(input);
    let bounds = (input.lines().count(), input.lines().next()?.len());
    let antinodes = get_antinodes(&antennas, bounds);
    Some(antinodes.iter().count())
}

fn get_antinodes(
    antennas: &HashMap<char, Vec<Pos<usize>>>,
    bounds: (usize, usize),
) -> HashSet<Pos<usize>> {
    let mut antinodes = HashSet::new();

    for (_frequency, positions) in antennas {
        for (&a, &b) in positions.iter().tuple_combinations() {
            if let Some(antinode) = (a * 2 - b).in_bounds(bounds) {
                antinodes.insert(antinode);
            }
            if let Some(antinode) = (b * 2 - a).in_bounds(bounds) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes
}

fn get_antenna_map(input: &str) -> HashMap<char, Vec<Pos<usize>>> {
    let mut antennas = HashMap::new();
    for (r, line) in input.lines().enumerate() {
        for (c, char) in line.char_indices() {
            if char != '.' {
                antennas
                    .entry(char)
                    .or_insert_with(Vec::new)
                    .push(Pos(r, c));
            }
        }
    }
    antennas
}

pub fn part_two(input: &str) -> Option<usize> {
    let antennas = get_antenna_map(input);
    let bounds = (input.lines().count(), input.lines().next()?.len());
    let antinodes = get_antinodes_2(&antennas, bounds);
    Some(antinodes.iter().count())
}

fn get_antinodes_2(
    antennas: &HashMap<char, Vec<Pos<usize>>>,
    bounds: (usize, usize),
) -> HashSet<Pos<usize>> {
    let mut antinodes = HashSet::new();

    for (_frequency, positions) in antennas {
        for (&a, &b) in positions.iter().tuple_combinations() {
            (0..)
                .map(|n| a + (b - a) * n)
                .map_while(|pos| pos.in_bounds(bounds))
                .for_each(|pos| {
                    antinodes.insert(pos);
                });

            (0..)
                .map(|n| b + (a - b) * n)
                .map_while(|pos| pos.in_bounds(bounds))
                .for_each(|pos| {
                    antinodes.insert(pos);
                });
        }
    }

    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_antenna_map_multiple_chars() {
        let input = "a.a\n..b\nc.a";
        let antennas = get_antenna_map(input);
        assert_eq!(antennas.len(), 3);
        assert_eq!(
            antennas.get(&'a'),
            Some(&vec![Pos(0, 0), Pos(0, 2), Pos(2, 2)])
        );
        assert_eq!(antennas.get(&'b'), Some(&vec![Pos(1, 2)]));
        assert_eq!(antennas.get(&'c'), Some(&vec![Pos(2, 0)]));
    }

    #[test]
    fn test_get_antenna_map_empty_input() {
        let input = "";
        let antennas = get_antenna_map(input);
        assert!(antennas.is_empty());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two(
            "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
",
        );
        assert_eq!(result, Some(9));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
