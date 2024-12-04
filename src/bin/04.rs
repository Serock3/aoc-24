use advent_of_code::parse_char_matrix;
use itertools::Itertools;
use ndarray::{s, Array2, ArrayView2, Ix2, SliceArg};
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_char_matrix(input);

    let rows: usize = matrix.rows().into_iter().map(find_xmas).sum();
    let cols: usize = matrix.columns().into_iter().map(find_xmas).sum();
    let (n_rows, n_cols) = matrix.dim();

    let lower_diag = find_diag_xmas((0..n_rows).map(|i| s![i.., ..]), &matrix);
    let upper_diag = find_diag_xmas((1..n_cols).map(|j| s![.., j..]), &matrix);

    let lower_diag_mirror = find_diag_xmas((0..n_rows).map(|i| s![i.., ..;-1]), &matrix);
    let upper_diag_mirror =
        find_diag_xmas((1..n_cols).map(|j| s![.., ..-(j as isize);-1]), &matrix);

    Some(rows + cols + lower_diag + upper_diag + lower_diag_mirror + upper_diag_mirror)
}

fn find_diag_xmas<S, I>(slice_iter: I, matrix: &Array2<char>) -> usize
where
    S: SliceArg<Ix2>,
    I: Iterator<Item = S>,
{
    slice_iter
        .map(|slice| matrix.slice(slice).into_diag())
        .map(find_xmas)
        .sum()
}

fn find_xmas<'a>(line: impl IntoIterator<Item = &'a char>) -> usize {
    line.into_iter()
        .tuple_windows()
        .filter(|(&a, &b, &c, &d)| {
            (a, b, c, d) == ('X', 'M', 'A', 'S') || (a, b, c, d) == ('S', 'A', 'M', 'X')
        })
        .count()
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse_char_matrix(input);
    Some(matrix.windows((3, 3)).into_iter().filter(has_x).count())
}

fn has_x(m3x3: &ArrayView2<char>) -> bool {
    (diag_is_mas(m3x3.slice(s![.., ..])) || diag_is_mas(m3x3.slice(s![..;-1, ..;-1])))
        && (diag_is_mas(m3x3.slice(s![.., ..;-1])) || diag_is_mas(m3x3.slice(s![..;-1, ..])))
}

fn diag_is_mas(diag: ArrayView2<char>) -> bool {
    diag.diag().iter().eq(['M', 'A', 'S'].iter())
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
        assert_eq!(result, Some(9));
    }
}
