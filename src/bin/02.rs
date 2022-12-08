use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let lines = input.lines();

    for line in lines {
        let opp_move = line.chars().nth(0).unwrap();
        let my_move = line.chars().nth(2).unwrap();
        // println!("Opponent move: {} and my move: {}", opp_move, my_move)

        total += match opp_move {
            'A' => match my_move {
                'X' => 4,
                'Y' => 8,
                'Z' => 3,
                _ => 0
            },
            'B' => match my_move {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0
            },
            'C' => match my_move {
                'X' => 7,
                'Y' => 2,
                'Z' => 6,
                _ => 0
            },
            _ => 0
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let lines = input.lines();

    for line in lines {
        let opp_move = line.chars().nth(0).unwrap();
        let my_move = line.chars().nth(2).unwrap();
        // println!("Opponent move: {} and my move: {}", opp_move, my_move)

        total += match opp_move {
            'A' => match my_move {
                'X' => 3,
                'Y' => 4,
                'Z' => 8,
                _ => 0
            },
            'B' => match my_move {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => 0
            },
            'C' => match my_move {
                'X' => 2,
                'Y' => 6,
                'Z' => 7,
                _ => 0
            },
            _ => 0
        }
    }

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
