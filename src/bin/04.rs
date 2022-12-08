use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let elf_pairings = lines.map(|line| {
        let range_assignments = line.split(['-', ',']);
        let borders = range_assignments.map(|border| border.parse::<u32>().unwrap());
        
        let tuple = borders.collect_tuple::<(u32,u32,u32,u32)>().unwrap();
        tuple
    });

    let colliding_elves: Vec<(u32, u32, u32, u32)> = elf_pairings.filter(|(a ,b,c,d)| {
        (a <= c && b >= d) || (c <= a && d >= b)
    }).collect();

    Some(colliding_elves.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let elf_pairings = lines.map(|line| {
        let range_assignments = line.split(['-', ',']);
        let borders = range_assignments.map(|border| border.parse::<u32>().unwrap());
        
        let tuple = borders.collect_tuple::<(u32,u32,u32,u32)>().unwrap();
        tuple
    });

    let colliding_elves: Vec<(u32, u32, u32, u32)> = elf_pairings.filter(|(a ,b,c,d)| {
        (a <= c && b >= c) || (c <= a && d >= a)
    }).collect();

    Some(colliding_elves.len().try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
