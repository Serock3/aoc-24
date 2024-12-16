use advent_of_code::{parse_char_matrix, Direction, Pos};
use itertools::Itertools;
use ndarray::{s, Array2};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let (mut matrix, dirs) = parse_input(input);
    let robot: Pos<isize> = matrix
        .indexed_iter()
        .find_map(|(idx, &c)| if c == '@' { Some(idx) } else { None })
        .map(|(r, c)| Pos(r as isize, c as isize))
        .unwrap();

    navigate_robot(dirs, &mut matrix, robot);
    Some(
        matrix
            .indexed_iter()
            .filter_map(|(idx, &c)| if c == 'O' { Some(idx) } else { None })
            .map(|(r, c)| r * 100 + c)
            .sum(),
    )
}

fn navigate_robot(dirs: Vec<Direction>, matrix: &mut Array2<char>, mut robot: Pos<isize>) {
    for dir in dirs {
        match matrix[(robot + dir).tuple()] {
            '.' => {
                matrix[robot.tuple()] = '.';
                robot += dir;
                matrix[robot.tuple()] = '@';
            }
            '#' => (),
            'O' => {
                let slice = match dir {
                    Direction::North => s![..=robot.0; -1, robot.1],
                    Direction::East => s![robot.0, robot.1..],
                    Direction::South => s![robot.0.., robot.1],
                    Direction::West => s![robot.0, ..=robot.1; -1],
                };
                if let Some(c) = matrix
                    .slice_mut(slice)
                    .iter_mut()
                    .find(|c| **c == '.' || **c == '#')
                {
                    if *c == '.' {
                        *c = 'O';
                        matrix[robot.tuple()] = '.';
                        robot += dir;
                        matrix[robot.tuple()] = '@';
                    }
                };
            }
            _ => panic!("Invalid robot position"),
        };
    }
}

pub fn parse_input(input: &str) -> (Array2<char>, Vec<Direction>) {
    let (matrix_str, moves_str) = input.split_once("\n\n").unwrap();
    let matrix = parse_char_matrix(matrix_str);
    let dirs = moves_str
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| Direction::from_char(c).unwrap())
        .collect();
    (matrix, dirs)
}

pub fn parse_input_2(input: &str) -> (Array2<char>, Vec<Direction>) {
    let (matrix_str, moves_str) = input.split_once("\n\n").unwrap();
    let matrix = part_char_array_part2(matrix_str);
    let dirs = moves_str
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| Direction::from_char(c).unwrap())
        .collect();
    (matrix, dirs)
}

/// This function is a copy of the `parse_char_matrix` function from the lib, but includes the
/// modified interpretation of the characters of part 2.
fn part_char_array_part2(
    matrix_str: &str,
) -> ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>> {
    let cols = matrix_str
        .lines()
        .map(|l| l.len())
        .reduce(|prev, next| {
            assert_eq!(prev, next);
            next
        })
        .unwrap();
    let rows = matrix_str.lines().count();
    let shape = (rows, cols * 2);
    let chars = matrix_str.chars().filter(|c| *c != '\n').flat_map(|c| {
        match c {
            '#' => "##",
            '.' => "..",
            '@' => "@.",
            'O' => "[]",
            _ => unreachable!(),
        }
        .chars()
    });
    ndarray::Array::from_iter(chars)
        .into_shape_clone((shape, ndarray::Order::RowMajor))
        .unwrap()
}

fn navigate_robot_2(dirs: Vec<Direction>, matrix: &mut Array2<char>, mut robot: Pos<isize>) {
    for dir in dirs {
        match matrix[(robot + dir).tuple()] {
            '.' => {
                matrix[robot.tuple()] = '.';
                robot += dir;
                matrix[robot.tuple()] = '@';
            }
            '#' => (),
            '[' => {
                if let Some(movable_boxes) = box_moved(matrix, robot + dir, dir) {
                    for box_pos_left in movable_boxes.into_iter().unique() {
                        let box_pos_right = box_pos_left + Direction::East;
                        matrix[(box_pos_right + dir).tuple()] = ']';
                        matrix[box_pos_right.tuple()] = '.';
                        matrix[(box_pos_left + dir).tuple()] = '[';
                        matrix[box_pos_left.tuple()] = '.';
                    }
                    matrix[robot.tuple()] = '.';
                    robot += dir;
                    matrix[robot.tuple()] = '@';
                }
            }
            ']' => {
                if let Some(movable_boxes) = box_moved(matrix, robot + dir + Direction::West, dir) {
                    for box_pos_left in movable_boxes.into_iter().unique() {
                        let box_pos_right = box_pos_left + Direction::East;
                        matrix[(box_pos_left + dir).tuple()] = '[';
                        matrix[box_pos_left.tuple()] = '.';
                        matrix[(box_pos_right + dir).tuple()] = ']';
                        matrix[box_pos_right.tuple()] = '.';
                    }
                    matrix[robot.tuple()] = '.';
                    robot += dir;
                    matrix[robot.tuple()] = '@';
                }
            }
            c => unreachable!("Invalid character: {:?}", c),
        };
    }
}

/// If the given box can be moved in the given direction, returns the (left side) positions of all
/// the boxes that will be moved, otherwise returns None.
fn box_moved(
    matrix: &Array2<char>,
    box_left_side: Pos<isize>,
    dir: Direction,
) -> Option<Vec<Pos<isize>>> {
    let box_right_side = box_left_side + Direction::East;

    match dir {
        Direction::North | Direction::South => {
            match (
                matrix[(box_left_side + dir).tuple()],
                matrix[(box_right_side + dir).tuple()],
            ) {
                ('.', '.') => Some(vec![box_left_side]),
                ('#', _) => None,
                (_, '#') => None,
                ('[', ']') => box_moved(matrix, box_left_side + dir, dir).map(|mut v| {
                    v.push(box_left_side);
                    v
                }),
                (']', '.') => {
                    box_moved(matrix, box_left_side + dir + Direction::West, dir).map(|mut v| {
                        v.push(box_left_side);
                        v
                    })
                }
                ('.', '[') => box_moved(matrix, box_right_side + dir, dir).map(|mut v| {
                    v.push(box_left_side);
                    v
                }),
                (']', '[') => {
                    let left_box = box_moved(matrix, box_left_side + dir + Direction::West, dir)?;
                    let right_box = box_moved(matrix, box_right_side + dir, dir)?;
                    Some([left_box, right_box, vec![box_left_side]].concat())
                }
                c => unreachable!("Invalid character: {:?}", c),
            }
        }
        Direction::East => match matrix[(box_right_side + dir).tuple()] {
            '.' => Some(vec![box_left_side]),
            '#' => None,
            '[' => box_moved(matrix, box_right_side + dir, dir).map(|mut v| {
                v.push(box_left_side);
                v
            }),
            c => unreachable!("Invalid character: {:?}", c),
        },
        Direction::West => match matrix[(box_left_side + dir).tuple()] {
            '.' => Some(vec![box_left_side]),
            '#' => None,
            ']' => box_moved(matrix, box_left_side + dir + dir, dir).map(|mut v| {
                v.push(box_left_side);
                v
            }),
            c => unreachable!("Invalid character: {:?}", c),
        },
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut matrix, dirs) = parse_input_2(input);

    let robot: Pos<isize> = matrix
        .indexed_iter()
        .find_map(|(idx, &c)| if c == '@' { Some(idx) } else { None })
        .map(|(r, c)| Pos(r as isize, c as isize))
        .unwrap();

    navigate_robot_2(dirs, &mut matrix, robot);
    Some(
        matrix
            .indexed_iter()
            .filter_map(|(idx, &c)| if c == '[' { Some(idx) } else { None })
            .map(|(r, c)| r * 100 + c)
            .sum(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    const SMALL_EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_part_one_small() {
        let result = part_one(SMALL_EXAMPLE);
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    const SMALL_EXAMPLE_2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
    #[test]
    fn test_part_two_small() {
        let result = part_two(SMALL_EXAMPLE_2);
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
