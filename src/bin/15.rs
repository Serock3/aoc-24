use advent_of_code::{parse_char_matrix, print_matrix, Direction, Pos};
use ndarray::{s, Array2};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let (mut matrix, dirs) = parse_input(input);
    // println!("Initial matrix:");
    // print_matrix(&matrix);
    // println!();
    // println!();
    // println!("Directions: {:?}", dirs);

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
        // println!("Moving {:?}:", dir);
        // print_matrix(&matrix);
        // println!();
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
                if box_moved(matrix, robot + dir, dir) {
                    matrix[robot.tuple()] = '.';
                    robot += dir;
                    matrix[robot.tuple()] = '@';
                }
            }
            ']' => {
                if box_moved(matrix, robot + dir + Direction::West, dir) {
                    matrix[robot.tuple()] = '.';
                    robot += dir;
                    matrix[robot.tuple()] = '@';
                }
            }

            _ => panic!("Invalid robot position"),
        };
        // println!("Moving {:?}:", dir);
        // print_matrix(&matrix);
        // println!();
    }
}

fn box_moved(matrix: &mut Array2<char>, box_left_side: Pos<isize>, dir: Direction) -> bool {
    let box_left_side_new = box_left_side + dir;
    let box_right_side = box_left_side + Direction::East;
    let box_right_side_new = box_right_side + dir;

    // I was hoping to solve this with recursion, but now that I think about it i'm not sure if
    // it'll work. If encountering two new boxes, the function will be called twice, and both
    // versions would need to know if the other one encountered a wall before moving the box.
    // Perhaps the correct solution is to not mutate the matrix directly in the function, which now
    // that I think about it will never be allowed in Rust anyway, but instead return information
    // about what boxes can be moved to the outermost
    todo!()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut matrix, dirs) = parse_input(input);
    // println!("Initial matrix:");
    // print_matrix(&matrix);
    // println!();
    // println!();
    // println!("Directions: {:?}", dirs);

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

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
