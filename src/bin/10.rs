use itertools::Itertools;

pub fn input_to_instructions_vec(input: &str) -> Vec<&str> {
    input.lines().into_iter().collect_vec()
}

fn tick(cycles: &mut i32, signal_strengths: &mut Vec<i32>, x: i32) {
    *cycles += 1;
    push_signal_strength(*cycles, signal_strengths, x);
}

fn push_signal_strength(cycles: i32, signal_strengths: &mut Vec<i32>, x: i32) {
    if [20, 60, 100, 140, 180, 220].contains(&cycles) {
            signal_strengths.push(x * cycles);
            // println!("Ammount of cycles: {} Current strenght: {}", cycles, x * cycles);
    }
}

fn do_crt_tick(cycles: &mut i32, x: i32, crt_output: &mut String) {
    push_output(cycles, x, crt_output);
    *cycles += 1;
}

fn push_output(cycles: &mut i32, x: i32, crt_output: &mut String) {
    let row_cycle = *cycles % 40;
    if x - 1 <= row_cycle && row_cycle <= x + 1 {
        crt_output.push('#');
    } else {
        crt_output.push('.');
    }
    if (*cycles + 1) % 40 == 0 {
        crt_output.push('\n')
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = input_to_instructions_vec(input);


    let mut x = 1;
    let mut cycles = 0;
    let mut signal_strengths = Vec::new(); 

    for instruction in instructions {
        tick(&mut cycles, &mut signal_strengths, x);

        match instruction.split_whitespace().collect_vec()[..] {
            [_, add] => {
                tick(&mut cycles, &mut signal_strengths, x);
                x += add.parse::<i32>().unwrap();
            },
            _ => {}            
        }
    }

    Some(signal_strengths.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions = input_to_instructions_vec(input);

    let mut x = 1;
    let mut cycles = 0;

    let mut crt_output = "".to_owned();

    for instruction in instructions {
        do_crt_tick(&mut cycles, x, &mut crt_output);

        match instruction.split_whitespace().collect_vec()[..] {
            [_, add] => {
                do_crt_tick(&mut cycles, x, &mut crt_output);
                x += add.parse::<i32>().unwrap();
            },
            _ => {}            
        }
    }

    Some(crt_output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(
            "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n"
                .to_owned()
        ));
    }
}
