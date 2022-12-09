use std::collections::BTreeSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    Some(overlap_counter(lines.collect_vec(), 2))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    Some(overlap_counter(lines.collect_vec(), 10))
}

fn overlap_counter(lines: Vec<&str>, number_of_knots: usize) -> u32 {
    let mut knots = vec![(0, 0); number_of_knots];

    let mut tail_visited: BTreeSet<(i32, i32)> = BTreeSet::new();
    tail_visited.insert(*knots.last().unwrap());

    lines.into_iter().for_each(|line| {
        let (direction, number_of_steps) = line.trim().split_once(' ').unwrap();

        (0..number_of_steps.parse().unwrap()).for_each(|_| {
            match direction {
                "L" => knots[0].0 -= 1,
                "R" => knots[0].0 += 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => {}
            }

            (1..number_of_knots).for_each(|knot| {
                if (knots[knot - 1].0 - knots[knot].0).abs() >= 2
                    || (knots[knot - 1].1 - knots[knot].1).abs() >= 2
                {
                    knots[knot].0 += (knots[knot - 1].0 - knots[knot].0).signum();
                    knots[knot].1 += (knots[knot - 1].1 - knots[knot].1).signum();
                }
            });

            tail_visited.insert(*knots.last().unwrap());
        });
    });

    tail_visited.len() as u32
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
