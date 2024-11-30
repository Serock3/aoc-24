use std::collections::VecDeque;

use advent_of_code::{parse_char_matrix, parse_int_matrix, Direction, Pos, DIRECTIONS};
use hashbrown::HashMap;
use ndarray::Array2;
advent_of_code::solution!(17);
// 1. 872 too high

type Int = usize;

type Key = (Pos<Int>, Direction, u8); // u8 is the number of of steps in a straight line

pub fn part_one(input: &str) -> Option<Int> {
    let heat_matrix = parse_int_matrix::<Int>(input);
    let [rows, cols] = heat_matrix.shape().try_into().unwrap();
    println!("{}", heat_matrix);

    let end = Pos(rows - 1, cols - 1);
    let mut min_heat = HashMap::<Key, Int>::with_capacity(1024);

    let start_east = (Pos(0, 1), Direction::East, 1);
    min_heat.insert(start_east, get_heat(&heat_matrix, Pos(0, 1)));
    let start_south = (Pos(1, 0), Direction::South, 1);
    min_heat.insert(start_south, get_heat(&heat_matrix, Pos(1, 0)));

    let mut queue = VecDeque::<Key>::new();
    queue.push_back(start_east);
    queue.push_back(start_south);
    // let mut came_from = HashMap::<PosDir, PosDir>::with_capacity(1024);
    // came_from.insert((start, Direction::East), (start, Direction::East));
    // came_from.insert((start, Direction::South), (start, Direction::South));

    while let Some(key) = queue.pop_front() {
        let heat_to_current_pos = min_heat[&key];
        for next_dir in DIRECTIONS {
            let (pos, dir, straight_steps) = key;
            let Some(next_pos) = (pos + next_dir).in_bounds([rows, cols]) else {
                continue;
            };

            // Skip if we are moving back
            if Pos::from(dir) + Pos::from(next_dir) == Pos(0, 0) {
                continue;
            }

            let next_straight_steps = if dir == next_dir {
                straight_steps + 1
            } else {
                1
            };
            if next_straight_steps > 3 {
                assert_eq!(next_straight_steps, 4);
                continue;
            }

            // // Skip if we have traveled in a straight line for more than 3 steps
            // let prev_pos = came_from.get(&pos).unwrap();
            // // Don't allow 180 degree turns
            // if prev_pos == &next_pos {
            //     continue;
            // }
            // let prev_pos = came_from.get(prev_pos).unwrap();
            // let prev_pos = came_from.get(prev_pos).unwrap();
            // match next_pos - *prev_pos {
            //     Pos(x, 0) if x.abs() >= 4 => continue,
            //     Pos(0, y) if y.abs() >= 4 => continue,
            //     _ => {}
            // }

            let tentative_dist = heat_to_current_pos + get_heat(&heat_matrix, next_pos);
            if next_pos == end {
                return Some(tentative_dist);
            }
            let next_key = (next_pos, next_dir, next_straight_steps);
            if min_heat
                .get(&next_key)
                .map_or(true, |&old_dist| tentative_dist < old_dist)
            {
                min_heat.insert(next_key, tentative_dist);
                // came_from.insert(next_pos, pos);
                queue.push_back(next_key);
            }
        }
    }
    panic!("No path found");

    // let mut char_matrix = parse_char_matrix(input);
    // // let mut path = vec![end];
    // let mut current = end;
    // while current != start {
    //     *char_matrix.get_mut::<(Int, Int)>(current.into()).unwrap() = '#';
    //     current = came_from[&current];
    //     // path.push(current);
    // }
    // println!("\n{char_matrix}");
    // path.reverse();
    // println!("Path: {:?}", path);

    // Some(dist[&end])
}

fn get_heat(matrix: &Array2<usize>, next_pos: Pos<usize>) -> usize {
    *matrix.get(next_pos.tuple()).unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
