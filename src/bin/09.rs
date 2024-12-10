use std::{
    collections::BTreeMap,
    iter::{self, DoubleEndedIterator},
};

use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let nums = parse_nums(input);
    let fragments = make_fragments(&nums);

    let total_size = nums.iter().sum::<usize>();
    let compacted = compact(fragments, total_size).collect_vec();
    calc_checksum(compacted).into()
}

/// Calculate the checksum of the compacted memory
///
/// Takes an iterator of usize or Option<usize>, for convenience
fn calc_checksum<T: Into<Option<usize>>>(compacted: impl IntoIterator<Item = T>) -> usize {
    compacted
        .into_iter()
        .enumerate()
        .flat_map(|(pos, id)| id.into().map(|id| pos * id))
        .sum::<usize>()
}

fn parse_nums(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec()
}

fn compact<'a>(
    fragments: impl Clone + DoubleEndedIterator<Item = Option<usize>> + 'a,
    total_size: usize,
) -> impl Iterator<Item = usize> + 'a {
    // iterator through the fragments in memory backward that skips the empty spaces
    let mut backwards = fragments
        .clone()
        .rev()
        .enumerate()
        .filter_map(move |(pos, id)| id.map(|id| (total_size - pos, id)))
        .peekable();

    // iterator through the fragments in memory forward
    let forwards = fragments.enumerate();
    forwards.map_while(move |(pos, id)| {
        let pos_back = backwards.peek().unwrap().0;
        if pos_back <= pos {
            None
        } else {
            match id {
                Some(id) => Some(id),
                None => {
                    let (_, id_back) = backwards.next().unwrap();
                    Some(id_back)
                }
            }
        }
    })
}

fn make_fragments(nums: &[usize]) -> impl Clone + DoubleEndedIterator<Item = Option<usize>> + '_ {
    nums.chunks(2).enumerate().flat_map(|(id, slice)| {
        let frag_size = slice[0];
        let fragment = iter::repeat_n(Some(id), frag_size);
        let free_size = slice[1];
        let free_space = iter::repeat_n(None, free_size);

        fragment.chain(free_space)
    })
}

// part 2

#[derive(Debug, Clone, Eq, PartialEq)]
struct Fragment {
    size: usize,
    id: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum MemSegment {
    Fragment(Fragment),
    Free { size: usize },
}

pub fn part_two(input: &str) -> Option<usize> {
    let nums = parse_nums(input);
    let mut position = 0;
    let mut segments: BTreeMap<usize, MemSegment> = nums
        .chunks(2)
        .enumerate()
        .flat_map(|(id, slice)| {
            let frag_start = position;
            let fragment = MemSegment::Fragment(Fragment { size: slice[0], id });
            position += slice[0];
            let free_space_start = position;
            let free_space = MemSegment::Free { size: slice[1] };
            position += slice[1];
            vec![(frag_start, fragment), (free_space_start, free_space)]
        })
        .collect();

    let backwards = segments
        .clone()
        .into_iter()
        .rev()
        .filter_map(|(start, seg)| match seg {
            MemSegment::Fragment(fragment) => Some((start, fragment)),
            MemSegment::Free { .. } => None,
        })
        .peekable();

    for (start, fragment) in backwards {
        let mut right = segments.split_off(&start);

        let find_map = segments.iter().find_map(|(pos, seg)| match seg {
            MemSegment::Fragment(_) => None,
            MemSegment::Free { size } if *size >= fragment.size => Some((*pos, *size)),
            _ => None,
        });
        if let Some((free_space_pos, free_space_size)) = find_map {
            // Remove the fragment from the right half, replace with free space
            right.remove(&start).unwrap();
            right.insert(
                start,
                MemSegment::Free {
                    size: fragment.size,
                },
            );

            // Remove the free space from the left half, replace with fragment and, maybe, smaller free space
            segments.remove(&free_space_pos).unwrap();
            segments.insert(free_space_pos, MemSegment::Fragment(fragment.clone()));
            if free_space_size > fragment.size {
                segments.insert(
                    free_space_pos + fragment.size,
                    MemSegment::Free {
                        size: free_space_size - fragment.size,
                    },
                );
            }
        };
        segments.extend(right);
    }

    // segments.iter().for_each(|(pos, seg)| {
    //     println!("{:?} {:?}", pos, seg);
    // });

    let compacted: Vec<Option<usize>> = segments
        .iter()
        .flat_map(|(_, seg)| match seg {
            MemSegment::Fragment(fragment) => iter::repeat_n(Some(fragment.id), fragment.size),
            MemSegment::Free { size } => iter::repeat_n(None, *size),
        })
        .collect();
    Some(calc_checksum(compacted))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("123450", "0..111....22222")]
    #[case("23331331214141314020", "00...111...2...333.44.5555.6666.777.888899")]
    fn test_make_fragments(#[case] input: &str, #[case] expected: &str) {
        let nums = parse_nums(input);
        let fragments = make_fragments(&nums);
        let result = fragments
            .map(|id| match id {
                Some(n) => std::char::from_digit(n as u32, 10).unwrap(),
                None => '.',
            })
            .collect::<String>();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("0099811188827773336446555566", 1928)]
    fn test_calc_checksum(#[case] input: &str, #[case] expected: usize) {
        let nums: Vec<usize> = parse_nums(input);
        let result = calc_checksum(nums);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("123450", "022111222")]
    #[case("23331331214141314020", "0099811188827773336446555566")]
    fn test_part_one_small(#[case] input: &str, #[case] compacted: &str) {
        let result = part_one(input);
        let nums: Vec<usize> = parse_nums(compacted);
        let expected = calc_checksum(nums);
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_one() {
        let result = part_one("23331331214141314020");
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("23331331214141314020");
        assert_eq!(result, Some(2858));
    }
}
