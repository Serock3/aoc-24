use advent_of_code::parse_char_matrix;
use ndarray::s;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_char_matrix(input);
    let rows: usize = dbg!(matrix.rows().into_iter().map(find_xmas_windows).sum());
    let cols: usize = dbg!(matrix.columns().into_iter().map(find_xmas_windows).sum());
    matrix
        .slice(s![..;4..])
        .diag()
        .into_iter()
        .map(find_xmas_windows)
        .sum();
    Some(rows + cols)
}

fn find_xmas_windows(
    row: ndarray::ArrayBase<ndarray::ViewRepr<&char>, ndarray::Dim<[usize; 1]>>,
) -> usize {
    println!("{}", row);
    dbg!(row
        .windows(4)
        .into_iter()
        .filter(|window| {
            println!("{}", window);
            dbg!(window.iter().cloned().eq("XMAS".chars()))
                || dbg!(window.iter().rev().cloned().eq("XMAS".chars()))
        })
        .count())
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
