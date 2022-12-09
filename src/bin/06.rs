use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let char_vec = input.trim().chars().collect_vec();
    let result = char_vec
        .windows(4)
        .enumerate()
        .filter(|(_, window)| window.into_iter().all_unique())
        .map(|(i, _)| i + 4)
        .next()
        .unwrap() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_vec = input.trim().chars().collect_vec();
    let result = char_vec
        .windows(14)
        .enumerate()
        .filter(|(_, window)| window.into_iter().all_unique())
        .map(|(i, _)| i + 14)
        .next()
        .unwrap() as u32;

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
