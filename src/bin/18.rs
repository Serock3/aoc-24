use std::collections::VecDeque;

use advent_of_code::{Pos, DIRECTIONS};
use hashbrown::{HashMap, HashSet};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let shape = 70;
    Some(solve1(input, shape, 1024))
}

fn solve1(input: &str, shape: usize, bytes_fallen: usize) -> usize {
    let corrupted_bytes: Vec<Pos<usize>> = input
        .lines()
        .take(bytes_fallen)
        .map(|s| {
            let (r, c) = s.split_once(',').unwrap();
            Pos(r.parse().unwrap(), c.parse().unwrap())
        })
        .collect();

    solve_maze(shape, HashSet::from_iter(&corrupted_bytes)).unwrap()
}

fn solve_maze(shape: usize, corrupted_bytes: HashSet<&Pos<usize>>) -> Option<usize> {
    let start_pos = Pos(0, 0);
    let end_pos = Pos(shape, shape);
    let mut queue = VecDeque::<Pos<usize>>::new();
    queue.push_back(start_pos);
    let mut min_dist = HashMap::new();
    min_dist.insert(Pos(0, 0), 0);
    while let Some(pos) = queue.pop_front() {
        let dist = min_dist[&pos];
        for dir in DIRECTIONS {
            let Some(next_pos) = (pos + dir).in_bounds([shape + 1, shape + 1]) else {
                continue;
            };
            if corrupted_bytes.contains(&next_pos) {
                continue;
            }
            let next_dist = dist + 1;
            if next_pos == end_pos {
                // debug_print_maze(shape, &corrupted_bytes);
                return Some(next_dist);
            }
            if min_dist
                .get(&next_pos)
                .map_or(true, |&old_dist| next_dist < old_dist)
            {
                min_dist.insert(next_pos, next_dist);
                queue.push_back(next_pos);
            }
        }
    }
    None
}

#[allow(dead_code)]
fn debug_print_maze(shape: usize, corrupted_bytes: &HashSet<&Pos<usize>>) {
    for x in 0..=shape {
        for y in 0..=shape {
            if corrupted_bytes.contains(&Pos(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<Pos<usize>> {
    let shape = 70;
    Some(solve2(input, shape))
}

fn solve2(input: &str, shape: usize) -> Pos<usize> {
    let corrupted_bytes_list: Vec<Pos<usize>> = input
        .lines()
        .map(|s| {
            let (r, c) = s.split_once(',').unwrap();
            Pos(r.parse().unwrap(), c.parse().unwrap())
        })
        .collect();
    let mut latest_known_solvable = 0;
    let mut earliest_known_unsolvable = corrupted_bytes_list.len();
    loop {
        if earliest_known_unsolvable == latest_known_solvable + 1 {
            return corrupted_bytes_list[latest_known_solvable];
        }
        let next_attempt = (latest_known_solvable + earliest_known_unsolvable) / 2;
        if solve_maze(
            shape,
            HashSet::from_iter(&corrupted_bytes_list[..next_attempt]),
        )
        .is_some()
        {
            latest_known_solvable = next_attempt;
        } else {
            earliest_known_unsolvable = next_attempt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part_one() {
        let result = solve1(INPUT, 6, 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_two() {
        let result = solve2(INPUT, 6);
        assert_eq!(result, Pos(6, 1));
    }
}
