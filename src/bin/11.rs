use core::str;

use itertools::Itertools;



#[derive(Clone)]
struct Monke {
    items: Vec<u64>,
    operation: Operation,
    test: Test
}
impl Monke {
    fn do_monke_bussiness(&mut self, current_index: usize, monkes: &mut Vec<Monke>, inspections: &mut Vec<u64>, verbose: bool) {
        let shared_multiple = monkes.iter().map(|m| m.test.divisible).product::<u64>();

        let items = &self.items.drain(..).collect_vec();

        if verbose {
            println!("Monkey {}:", current_index);
        }
        
        for old_worry_level in items {
            if verbose {
                println!("  Monkey inspects an item with a worry level of {}.", old_worry_level);
            }
            inspections[current_index] += 1;

            let new_worry_level = self.operation.calc_worry_level(*old_worry_level);
            let operation = if std::mem::discriminant(&self.operation) == std::mem::discriminant(&Operation::Add(Value::Old)) {"increases"} else {"is multiplied"};
            if verbose {
                println!("    Worry level {} by {} to {}.", operation, self.operation.get_old_worry_level(*old_worry_level), new_worry_level);
            }

            let new_worry_level = new_worry_level %  shared_multiple;
            if verbose {
                println!("    Monkey gets bored with item. Worry level mod {shared_multiple} to {}.", new_worry_level);
            }

            let new_worry_level = new_worry_level / 3;
            if verbose {
                println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", new_worry_level);
            }
            

            let is_divisible = new_worry_level % self.test.divisible == 0;
            let target_monke_index = if is_divisible {
                if verbose {
                    println!("    Current worry level is divisible by {}.", self.test.divisible);
                }
                self.test.true_monke
            } else {
                if verbose {
                    println!("    Current worry level is not divisible by {}.", self.test.divisible);
                }
                self.test.false_monke
            };

            let receiver = &mut monkes[target_monke_index as usize];
            receiver.items.push(new_worry_level);
            if verbose {
                println!("    Item with worry level {new_worry_level} is thrown to monkey {target_monke_index}.")
            }
        }

    }
}

#[derive(Clone)]
enum Operation {
    Multiply(Value),
    Add(Value)
}
impl Operation {
    fn calc_worry_level(&self, old_worry_level: u64) -> u64 {
        match &self {
            Operation::Add(value) => old_worry_level + value.get_worry_level(old_worry_level),
            Operation::Multiply(value) => old_worry_level * value.get_worry_level(old_worry_level),
        }
    }

    fn get_old_worry_level(&self, old_worry_level: u64) -> String {
        match &self {
            Operation::Add(value) => {
                if value.get_worry_level(old_worry_level) == old_worry_level { "itself".to_owned() } else { value.get_worry_level(old_worry_level).to_string() }
                },
            Operation::Multiply(value) => {
                if value.get_worry_level(old_worry_level) == old_worry_level { "itself".to_owned() } else { value.get_worry_level(old_worry_level).to_string() }
            }
        }
    }
}

#[derive(Clone)]
enum Value {
    Old,
    Num(u64) 
}
impl Value {
    fn get_worry_level(&self, old: u64) -> u64 {
        match self {
            Value::Num(number) => *number,
            Value::Old => old,
        }
    }
}

#[derive(Clone)]
struct Test {
    divisible: u64,
    true_monke: u64,
    false_monke: u64
}

fn lines_to_items(lines: &mut std::iter::Skip<str::Lines>) -> Vec<u64> {
    // Split second line at : {Starting items: 71, 72, 74, 76, 68}
    let (_, items) = lines.next().unwrap().split_once(": ").unwrap();
    // println!("Items: {}", items);
    let items = items.split(", ")
        .map(|item| item.parse::<u64>().unwrap())
        .collect_vec();
    // println!("Number of items: {}", items.len());
    items
}

fn line_to_operation(lines: &mut std::iter::Skip<str::Lines>) -> Operation {
    // Split third line at = old : {Operation: new = old + 8}
    let (_, operation) = lines.next().unwrap().split_once("= old ").unwrap();
    let (sign, value) = operation.split_once(" ").unwrap();
    let value = match value {
        "old" => Value::Old,
        _ => {
            Value::Num(value.parse::<u64>().unwrap())
        }
    };
    let operation = match sign {
        "+" => Operation::Add(value),
        "*" => Operation::Multiply(value),
        _ => panic!(),
    };
    operation
}

fn lines_to_test(mut lines: std::iter::Skip<str::Lines>) -> Test {
    let (_, divisible) = lines.next().unwrap().rsplit_once(" ").unwrap();
    let divisible = divisible.parse::<u64>().unwrap();
    let (_, true_monke) = lines.next().unwrap().rsplit_once(" ").unwrap();
    let true_monke = true_monke.parse::<u64>().unwrap();
    let (_, false_monke) = lines.next().unwrap().rsplit_once(" ").unwrap();
    let false_monke = false_monke.parse::<u64>().unwrap();
    let test = Test {
        divisible,
        true_monke,
        false_monke,
    };
    test
}

fn input_to_monkes(input: &str) -> Vec<Monke> {
    let mut monkes: Vec<Monke> = Vec::new();
    
    let input_groups = input.split("\n\n");
    for input_group in input_groups {
        // Skip the first line (Monkey 0:)
        let mut lines = input_group.lines().skip(1);
        
        let monke = Monke {
            items: lines_to_items(&mut lines),
            operation: line_to_operation(&mut lines),
            test: lines_to_test(lines),
        };

        monkes.push(monke)
    }
    monkes
}

fn return_to_monke(monkes: &mut Vec<Monke>, number_of_rounds: u32, verbose: bool) -> u64 {
    let mut monkeys_with_number_of_inspections = vec![0; monkes.len()];

    for round in 0..number_of_rounds {
        for index in 0..monkes.len() {
            let monke = &mut monkes[index].clone();
            monke.do_monke_bussiness(index, monkes, &mut monkeys_with_number_of_inspections, false);
            monkes[index] = monke.to_owned();
        }

        if verbose {
            println!("\nAfter round {}, the monkeys are holding items with these worry levels:", (round + 1));
        }
        
        for index in 0..monkes.len() {
            let monke = &monkes[index];
            let items = monke.items.iter().join(", ");
            if verbose {
                println!("Monkey {index}: {items}")
            }
        }
        if verbose {
            println!();
        }
    }
    
    if verbose {
        for (index, number_instpections) in monkeys_with_number_of_inspections.iter().enumerate() {
            println!("Monke {} inspected items {} times.", index, number_instpections);
        }
    }   

    monkeys_with_number_of_inspections.sort_unstable();
    monkeys_with_number_of_inspections.iter().rev().take(2).product::<u64>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkes = input_to_monkes(input);
    Some(return_to_monke(&mut monkes, 20, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkes = input_to_monkes(input);
    Some(return_to_monke(&mut monkes, 10000, false))

}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(3017072640));
    }
}
