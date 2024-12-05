use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (orderings, updates) = parse_input(input);

    Some(
        updates
            .filter(|update| correct_order(update, &orderings))
            .map(|update| get_middle_page(&update))
            .sum::<u32>(),
    )
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, impl Iterator<Item = Vec<u32>> + '_) {
    let (orderings_str, updates) = input.split_once("\n\n").unwrap();
    let orderings = orderings_str
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(page_a, page_b)| {
            (
                page_a.parse::<u32>().unwrap(),
                page_b.parse::<u32>().unwrap(),
            )
        })
        .collect_vec();

    let updates = updates.lines().map(|line| {
        line.split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec()
    });
    (orderings, updates)
}

fn correct_order(update: &[u32], orderings: &[(u32, u32)]) -> bool {
    orderings
        .iter()
        .map(|(page_a, page_b)| {
            let Some(page_a_index) = update.iter().position(|page| page == page_a) else {
                return true;
            };
            let Some(page_b_index) = update.iter().position(|page| page == page_b) else {
                return true;
            };
            page_a_index < page_b_index
        })
        .all(|b| b)
}

fn get_middle_page(pages: &[u32]) -> u32 {
    let len = pages.len();
    assert!(len % 2 == 1);
    let middle = len / 2;
    pages[middle]
}

pub fn part_two(input: &str) -> Option<u32> {
    let (orderings, updates) = parse_input(input);

    Some(
        updates
            .filter(|update| !correct_order(update, &orderings))
            .map(|update| sort(update, &orderings))
            .map(|update| get_middle_page(&update))
            .sum::<u32>(),
    )
}

fn sort(mut update: Vec<u32>, orderings: &[(u32, u32)]) -> Vec<u32> {
    update.sort_by(|a, b| {
        orderings
            .iter()
            .find_map(|order| match order {
                (page_a, page_b) if a == page_a && b == page_b => Some(std::cmp::Ordering::Less),
                (page_b, page_a) if a == page_a && b == page_b => Some(std::cmp::Ordering::Greater),
                _ => None,
            })
            .expect("Order missing")
    });
    update
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(&[75,47,61,53,29], 61)]
    #[case(&[97,61,53,29,13], 53)]
    #[case(&[75,29,13], 29)]
    fn test_middle_page(#[case] pages: &[u32], #[case] expected: u32) {
        let result = get_middle_page(pages);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
