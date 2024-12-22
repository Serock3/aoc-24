use std::iter;

use hashbrown::HashMap;
use itertools::Itertools;

advent_of_code::solution!(22);

type Int = i64;

pub fn part_one(input: &str) -> Option<Int> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<Int>().unwrap())
            .map(|mut num| {
                for _ in 0..2000 {
                    num = step(num);
                }
                num
            })
            .sum(),
    )
}

pub fn step(mut num: Int) -> Int {
    num = (num ^ (num * 64)) % 16777216;
    num = (num ^ (num / 32)) % 16777216;
    num = (num ^ (num * 2048)) % 16777216;
    num
}

pub fn part_two(input: &str) -> Option<Int> {
    let initial_secret_numbers = input.lines().map(|line| line.parse::<Int>().unwrap());
    let price_maps = initial_secret_numbers.map(|start_secret_num| {
        let num_secret_numbers = 2000;
        let secret_numbers = generate_secret_numbers(start_secret_num, num_secret_numbers + 1);
        let diffs = secret_numbers // step 2000 times, add one since this includes the initial value
            .tuple_windows()
            .map(|(a, b, c, d, e)| ((b - a, c - b, d - c, e - d), e));
        let mut diff_price_map = HashMap::new();
        for (diffs, price) in diffs {
            diff_price_map.entry(diffs).or_insert(price); // If we have not seen this diff-pattern
                                                          // before, insert the price it would
                                                          // result it
        }
        diff_price_map
    });

    let mut total_price_map = HashMap::new();
    for map in price_maps {
        for (diffs, price) in map {
            total_price_map
                .entry(diffs)
                .and_modify(|p| *p += price)
                .or_insert(price);
        }
    }
    // println!("{total_price_map:?}");
    let (diffs, price) = total_price_map
        .into_iter()
        .max_by_key(|(_diffs, price)| *price)
        .unwrap();
    println!("Optimal diff sequence: {diffs:?}");

    Some(price)
}

fn generate_secret_numbers(start: i64, amount: usize) -> impl Iterator<Item = i64> {
    iter::successors(Some(start), |secret_num| Some(step(*secret_num)))
        .map(|num| num % 10) // Get first digit
        .take(amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "1
10
100
2024
",
        );
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_secret_number_generator() {
        let result = generate_secret_numbers(123, 10).collect::<Vec<_>>();
        assert_eq!(result, vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "1
2
3
2024",
        );
        assert_eq!(result, Some(23));
    }
}
