use std::collections::VecDeque;

use advent_of_code::{parse_char_matrix, Pos, DIRECTIONS};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use ndarray::Array2;

advent_of_code::solution!(12);

struct Region {
    area: usize,
    perimeter: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_char_matrix(input);
    let mut seen = Array2::<bool>::default(matrix.dim());

    let mut total_price = 0;
    for (pos, c) in matrix.indexed_iter() {
        if seen[pos] {
            continue;
        }
        let region: Region = get_region(&matrix, pos, &mut seen, c);
        let price = region.area * region.perimeter;
        total_price += price;
    }

    Some(total_price)
}

fn get_region(
    matrix: &Array2<char>,
    pos: (usize, usize),
    seen_total: &mut Array2<bool>,
    c: &char,
) -> Region {
    let mut queue = VecDeque::from([pos]);
    let mut region = HashSet::new();
    seen_total[pos] = true;

    let mut area = 0;
    let mut perimeter = 0;

    while let Some(pos) = queue.pop_front() {
        region.insert(Pos::from(pos));

        let neighbors = DIRECTIONS
            .iter()
            .filter_map(|dir| (Pos::from(pos) + dir).in_bounds(matrix.dim()))
            .filter(|new_pos| region.contains(new_pos))
            .count();
        perimeter += 4;
        perimeter -= neighbors * 2;
        area += 1;

        for new_pos in DIRECTIONS
            .iter()
            .filter_map(|dir| (Pos::from(pos) + dir).in_bounds(matrix.dim()))
            .filter(|new_pos| matrix[new_pos.tuple()] == *c)
        {
            if seen_total[new_pos.tuple()] {
                continue;
            } else {
                seen_total[new_pos.tuple()] = true;
                queue.push_back(new_pos.tuple());
            }
        }
    }
    Region { area, perimeter }
}

struct Region2 {
    area: usize,
    sides: usize,
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse_char_matrix(input);
    let mut seen = Array2::<bool>::default(matrix.dim());

    let mut total_price = 0;
    for (pos, c) in matrix.indexed_iter() {
        if seen[pos] {
            continue;
        }
        let region: Region2 = get_region_2(&matrix, pos, &mut seen, c);
        let price = region.area * region.sides;
        total_price += price;
    }

    Some(total_price)
}

fn get_region_2(
    matrix: &Array2<char>,
    pos: (usize, usize),
    seen_total: &mut Array2<bool>,
    c: &char,
) -> Region2 {
    let mut queue = VecDeque::from([pos]);
    let mut region = HashSet::new();
    seen_total[pos] = true;

    let mut area = 0;
    let mut sides = 0;

    while let Some(pos) = queue.pop_front() {
        region.insert(Pos::from(pos));

        let neighbors = DIRECTIONS
            .iter()
            .filter_map(|dir| (Pos::from(pos) + dir).in_bounds(matrix.dim()))
            .filter(|new_pos| region.contains(new_pos))
            .count();

        let i_dont_even_know_tbh = {
            let adjacent_diagonals = DIRECTIONS
                .iter()
                .filter_map(|dir| {
                    (Pos::from(pos) + dir)
                        .in_bounds(matrix.dim())
                        .filter(|new_pos| region.contains(new_pos))
                        .map(|pos| (pos, dir))
                })
                .flat_map(|(pos, dir)| {
                    vec![
                        (pos + dir.turn_left()).in_bounds(matrix.dim()),
                        (pos + dir.turn_right()).in_bounds(matrix.dim()),
                    ]
                })
                .flatten()
                .collect_vec();

            let mut map = HashMap::<Pos<usize>, bool>::new();
            for e in adjacent_diagonals {
                map.entry(e).and_modify(|b| *b = true).or_insert(false);
            }
            map.iter_mut()
                .map(|(pos, b)| if *b { true } else { region.contains(pos) })
                .filter(|b| *b)
                .count()
        };

        sides += 4;
        sides -= neighbors * 4;
        sides += i_dont_even_know_tbh * 2;
        area += 1;

        for new_pos in DIRECTIONS
            .iter()
            .filter_map(|dir| (Pos::from(pos) + dir).in_bounds(matrix.dim()))
            .filter(|new_pos| matrix[new_pos.tuple()] == *c)
        {
            if seen_total[new_pos.tuple()] {
                continue;
            } else {
                seen_total[new_pos.tuple()] = true;
                queue.push_back(new_pos.tuple());
            }
        }
    }
    Region2 { area, sides }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two_get_region() {
        let input = "AAAA
BBCD
BBCC
EEEC
";
        let matrix = parse_char_matrix(input);
        let mut seen = Array2::<bool>::default(matrix.dim());
        let region = get_region_2(&matrix, (0, 0), &mut seen, &'A');
        assert_eq!(region.area, 4);
        assert_eq!(region.sides, 4);
        let region = get_region_2(&matrix, (1, 0), &mut seen, &'B');
        assert_eq!(region.area, 4);
        assert_eq!(region.sides, 4);
        let region = get_region_2(&matrix, (3, 0), &mut seen, &'E');
        assert_eq!(region.area, 3);
        assert_eq!(region.sides, 4);
        let region = get_region_2(&matrix, (1, 2), &mut seen, &'C');
        assert_eq!(region.area, 4);
        assert_eq!(region.sides, 8);
        let region = get_region_2(&matrix, (1, 3), &mut seen, &'D');
        assert_eq!(region.area, 1);
        assert_eq!(region.sides, 4);
    }

    #[test]
    fn test_part_two_get_region2() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
        let matrix = parse_char_matrix(input);
        let mut seen = Array2::<bool>::default(matrix.dim());
        let region = get_region_2(&matrix, (0, 0), &mut seen, &'E');
        assert_eq!(region.area, 17);
        assert_eq!(region.sides, 12);
        let region = get_region_2(&matrix, (1, 1), &mut seen, &'X');
        assert_eq!(region.area, 4);
        assert_eq!(region.sides, 4);
    }

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, Some(1206));
    }
}
