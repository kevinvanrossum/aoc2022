use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let sum_of_shared_items: u32 = lines
        .map(|line| {
            let (left, right) = line.as_bytes().split_at(line.len() / 2);
            let &shared_item = left
                .iter()
                .find(|left_item| right.contains(left_item))
                .unwrap();

            (if shared_item >= b'a' {
                1 + shared_item - b'a'
            } else {
                27 + shared_item - b'A'
            }) as u32
        })
        .sum();

    Some(sum_of_shared_items)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let rucksacks = lines.chunks(3);

    let duplicate_items = rucksacks.into_iter().map(|grouping| {
        let group_as_vec = grouping.collect_vec();
        group_as_vec[0]
            .bytes()
            .find(|item| {
                group_as_vec[1].bytes().contains(item) && group_as_vec[2].bytes().contains(item)
            })
            .unwrap()
    });

    let sum_of_shared_items = duplicate_items
        .map(|shared_item| {
            (if shared_item >= b'a' {
                1 + shared_item - b'a'
            } else {
                27 + shared_item - b'A'
            }) as u32
        })
        .sum();

    Some(sum_of_shared_items)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
