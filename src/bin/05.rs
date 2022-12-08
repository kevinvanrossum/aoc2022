use std::vec;

use itertools::Itertools;

struct Movement {
    amount: usize,
    from: usize,
    to:usize
}

pub fn part_one(input: &str) -> Option<String> {
    // Split at the double new line
    let (crate_input, instruction_input) = input.split("\n\n").into_iter().collect_tuple().unwrap();
    let (stack_lines, stack_numbers_string) = crate_input.rsplit_once('\n').unwrap();
    let number_of_stacks = stack_numbers_string.split_whitespace().last().unwrap().parse::<usize>().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_stacks];

    // println!("Stack numbers string: \n{}", stack_numbers_string);
    for line in stack_lines.lines().rev() {
        // println!("{}", line);
        for (index, mut char_chunk) in line.chars().chunks(4).into_iter().enumerate() {
            // println!("Char chunk: {}", char_chunk.join(""));
            let second_char = char_chunk.nth(1)?;
            // println!("second char: {}", second_char);
            if second_char.is_alphabetic() {
                stacks[index].push(second_char)
            }
        }
    }

    let mut movements: Vec<Movement> = Vec::new();
    for line in  instruction_input.lines() {
        let stripped_instruction = line.strip_prefix("move ");
        let (amount, stripped_instruction) = stripped_instruction.unwrap().split_once(" from ")?;
        let (from, to) = stripped_instruction.split_once(" to ")?;

        let movement = Movement {
            amount: amount.parse::<usize>().unwrap(),
            from: from.parse::<usize>().unwrap() -1,
            to: to.parse::<usize>().unwrap() -1,
        };

        movements.push(movement);
    }


    for movement in movements {
        let Movement{amount, from, to } = movement;
        


        for _step in 0..amount {
            if let Some(popped) = stacks[from].pop() {
                stacks[to].push(popped);
            }    
        }

    }

    let top_containers = stacks.iter()
        .filter_map(|stack| stack.iter().last())
        .collect::<String>();

    // println!("Crates: \n{}", crate_input);
    // println!("Instructions: \n{}", instruction_input);
    // println!("Stack Lines: \n{}", stack_lines);
    // println!("Stack numbers string: \n{}", stack_numbers_string);
    // println!("Number of stacks: {}", number_of_stacks);

    // println!("Top containers: {}", top_containers);

    return Some(top_containers);
}

pub fn part_two(input: &str) -> Option<String> {
    // Split at the double new line
    let (crate_input, instruction_input) = input.split("\n\n").into_iter().collect_tuple().unwrap();
    let (stack_lines, stack_numbers_string) = crate_input.rsplit_once('\n').unwrap();
    let number_of_stacks = stack_numbers_string.split_whitespace().last().unwrap().parse::<usize>().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_stacks];

    // println!("Stack numbers string: \n{}", stack_numbers_string);
    for line in stack_lines.lines().rev() {
        // println!("{}", line);
        for (index, mut char_chunk) in line.chars().chunks(4).into_iter().enumerate() {
            // println!("Char chunk: {}", char_chunk.join(""));
            let second_char = char_chunk.nth(1)?;
            // println!("second char: {}", second_char);
            if second_char.is_alphabetic() {
                stacks[index].push(second_char)
            }
        }
    }

    let mut movements: Vec<Movement> = Vec::new();
    for line in  instruction_input.lines() {
        let stripped_instruction = line.strip_prefix("move ");
        let (amount, stripped_instruction) = stripped_instruction.unwrap().split_once(" from ")?;
        let (from, to) = stripped_instruction.split_once(" to ")?;

        let movement = Movement {
            amount: amount.parse::<usize>().unwrap(),
            from: from.parse::<usize>().unwrap() -1,
            to: to.parse::<usize>().unwrap() -1,
        };

        movements.push(movement);
    }


    for movement in movements {
        let Movement{amount, from, to } = movement;
        let from_stacks_number_of_containers = stacks[from].len();
        let removed_containers = stacks[from].split_off(from_stacks_number_of_containers - amount);
        for removed_container in removed_containers {
            stacks[to].push(removed_container);
        }
    }

    let top_containers = stacks.iter()
        .filter_map(|stack| stack.iter().last())
        .collect::<String>();

    // println!("Crates: \n{}", crate_input);
    // println!("Instructions: \n{}", instruction_input);
    // println!("Stack Lines: \n{}", stack_lines);
    // println!("Stack numbers string: \n{}", stack_numbers_string);
    // println!("Number of stacks: {}", number_of_stacks);

    // println!("Top containers: {}", top_containers);

    return Some(top_containers);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
