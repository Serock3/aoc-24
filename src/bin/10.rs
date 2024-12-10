use std::collections::VecDeque;

use advent_of_code::{get_adjacent_positions, parse_int_matrix, Pos};
use hashbrown::HashSet;
use itertools::Itertools;
use ndarray::Array2;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_int_matrix::<usize>(input);
    let zeros = matrix
        .indexed_iter()
        .filter(|(_, &x)| x == 0)
        .map(|(i, _)| Pos::from(i))
        .collect_vec();

    zeros
        .iter()
        .map(|&zero| count_summits(zero, &matrix))
        .sum::<usize>()
        .into()
}

/// Count the number of summits reachable from the given position.
fn count_summits(zero: Pos<usize>, matrix: &Array2<usize>) -> usize {
    let mut seen = HashSet::<Pos<usize>>::from([zero]);
    let mut queue = VecDeque::<(Pos<usize>, usize)>::from([(zero, 0)]);
    let mut summits = 0;

    while let Some((current, height)) = queue.pop_front() {
        if height == 9 {
            summits += 1;
            continue;
        }
        let next = get_adjacent_positions(current, matrix.dim())
            .filter(|&p| matrix[(p.0, p.1)] == height + 1)
            .filter(|&p| !seen.contains(&p))
            .collect_vec();
        queue.extend(next.iter().map(|&p| (p, height + 1)));
        seen.extend(&next);
    }
    summits
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse_int_matrix::<usize>(input);
    let zeros = matrix
        .indexed_iter()
        .filter(|(_, &x)| x == 0)
        .map(|(i, _)| Pos::from(i))
        .collect_vec();

    zeros
        .iter()
        .map(|&zero| count_summits_2(zero, &matrix))
        .sum::<usize>()
        .into()
}

/// Same as `count_summits`, but without the `seen` set, which causes multiple routes to the same
/// summit to be counted.
fn count_summits_2(zero: Pos<usize>, matrix: &Array2<usize>) -> usize {
    let mut queue = VecDeque::<(Pos<usize>, usize)>::from([(zero, 0)]);
    let mut summits = 0;

    while let Some((current, height)) = queue.pop_front() {
        if height == 9 {
            summits += 1;
            continue;
        }
        let next = get_adjacent_positions(current, matrix.dim())
            .filter(|&p| matrix[(p.0, p.1)] == height + 1)
            .collect_vec();
        queue.extend(next.iter().map(|&p| (p, height + 1)));
    }
    summits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(result, Some(81));
    }
}
