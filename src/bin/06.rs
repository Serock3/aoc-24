use advent_of_code::{parse_char_matrix, Direction, Pos};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_char_matrix(input);
    let pos = find_start_pos(&matrix).unwrap();
    let dir = Direction::North;

    let mut seen = hashbrown::HashSet::new();
    seen.insert(pos);

    let seen = trace_path(pos, dir, &matrix, seen);

    Some(seen.iter().count())
}

fn trace_path(
    mut pos: Pos<usize>,
    mut dir: Direction,
    matrix: &ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>>,
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

fn is_loop(
    mut pos: Pos<usize>,
    mut dir: Direction,
    matrix: &ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>>,
    mut seen: hashbrown::HashSet<Pos<usize>>,
) -> bool {
    loop {
        let Some(next_pos) = (pos + dir).in_bounds(matrix.dim()) else {
            return false;
        };

        match matrix[[next_pos.0, next_pos.1]] {
            '#' => {
                if seen.contains(&pos) {
                    println!("{:?} {:?}", pos, dir);
                    return true;
                }
                dir = dir.turn_right()
            }
            '.' | '^' => {
                pos = next_pos;
                seen.insert(pos);
            }
            _ => unreachable!(),
        }
    }
}

fn find_start_pos(
    matrix: &ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>>,
) -> Option<Pos<usize>> {
    matrix
        .rows()
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .find_map(|(j, c)| (c == &'^').then_some(Pos(i, j)))
        })
        .find_map(|c| c)
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse_char_matrix(input);
    let pos = find_start_pos(&matrix).unwrap();
    let dir = Direction::North;

    let mut seen = hashbrown::HashSet::new();
    seen.insert(pos);

    let seen = trace_path(pos, dir, &matrix, seen);

    Some(
        seen.into_iter()
            .skip(1)
            .map(|p| {
                let mut m = matrix.clone();
                m[[p.0, p.1]] = '#';
                is_loop(p, dir, &m, hashbrown::HashSet::new())
            })
            .count(),
    )
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
