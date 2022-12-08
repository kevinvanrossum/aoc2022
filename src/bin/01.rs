use itertools::Itertools;

pub fn get_totals(input: &str) -> Vec<u32> {
    input.split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|food| food.parse::<u32>().unwrap_or(0)).sum()
        })
        .sorted()
        .rev()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_totals(input)[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_totals(input)
        .iter()
        .take(3)
        .sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
