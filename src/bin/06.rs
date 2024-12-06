use advent_of_code::{parse_char_matrix, Direction, Pos};
use ndarray::Array2;

// 2142 too low
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_char_matrix(input);
    let pos = find_start_pos(&matrix);
    let dir = Direction::North;

    let mut seen = hashbrown::HashSet::new();
    seen.insert(pos);

    let seen = trace_path(pos, dir, &matrix, seen);

    Some(seen.iter().count())
}

fn trace_path(
    mut pos: Pos<usize>,
    mut dir: Direction,
    matrix: &Array2<char>,
    mut seen: hashbrown::HashSet<Pos<usize>>,
) -> hashbrown::HashSet<Pos<usize>> {
    loop {
        let Some(next_pos) = (pos + dir).in_bounds(matrix.dim()) else {
            return seen;
        };

        match matrix[[next_pos.0, next_pos.1]] {
            '#' => dir = dir.turn_right(),
            '.' | '^' => pos = next_pos,
            _ => unreachable!(),
        }

        seen.insert(pos);
    }
}

fn find_start_pos(matrix: &Array2<char>) -> Pos<usize> {
    let (rows, cols) = matrix.dim();

    for r in 0..rows {
        for c in 0..cols {
            if matrix[[r, c]] == '^' {
                return Pos(r, c);
            }
        }
    }
    unreachable!()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut matrix = parse_char_matrix(input);
    let start_pos = find_start_pos(&matrix);
    let dir = Direction::North;

    let mut seen = hashbrown::HashSet::new();
    seen.insert(start_pos);

    let seen = trace_path(start_pos, dir, &matrix, seen);
    matrix[[start_pos.0, start_pos.1]] = '^';

    Some(
        seen.into_iter()
            .skip(1)
            .filter(|box_pos| {
                let mut matrix = matrix.clone();
                matrix[[box_pos.0, box_pos.1]] = 'O';

                is_loop(start_pos, dir, matrix)
            })
            .count(),
    )
}

fn is_loop(mut start_pos: Pos<usize>, mut dir: Direction, matrix: Array2<char>) -> bool {
    let mut seen = hashbrown::HashSet::new();
    loop {
        seen.insert((start_pos, dir));

        let Some(next_pos) = (start_pos + dir).in_bounds(matrix.dim()) else {
            return false;
        };

        // match dir {
        //     Direction::North | Direction::South => matrix[[start_pos.0, start_pos.1]] = '|',
        //     Direction::East | Direction::West => matrix[[start_pos.0, start_pos.1]] = '-',
        // }

        match matrix[[next_pos.0, next_pos.1]] {
            '#' | 'O' => dir = dir.turn_right(),
            '.' | '^' | '|' | '-' => {
                start_pos = next_pos;
            }
            _ => unreachable!(),
        }

        if seen.contains(&(start_pos, dir)) {
            // print_matrix(&matrix);
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
