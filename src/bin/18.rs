use std::collections::VecDeque;

use advent_of_code::{Pos, DIRECTIONS};
use hashbrown::{HashMap, HashSet};
use ndarray::Array2;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    let shape = 70;
    Some(solve1(input, shape, 1024))
}

fn solve1(input: &str, shape: usize, bytes_fallen: usize) -> usize {
    // let matrix = Array2::zeros((shape, shape));
    let corrupted_bytes: HashSet<Pos<usize>> = input
        .lines()
        .take(bytes_fallen)
        .map(|s| {
            let (r, c) = s.split_once(',').unwrap();
            Pos(r.parse().unwrap(), c.parse().unwrap())
        })
        .collect();
    let end_pos = Pos(shape, shape);
    let start_pos = Pos(0, 0);

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
                return next_dist;
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
    unreachable!()
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
        let result = part_two(INPUT);
        assert_eq!(result, None);
    }
}
