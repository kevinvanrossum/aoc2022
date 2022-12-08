use std::{collections::HashMap, path::{PathBuf, Path}};

fn input_to_sizes(input: &str) -> HashMap<PathBuf, u32> {
    let lines = input.lines();
    let mut paths_with_sizes = HashMap::new();
    let mut visited_paths = Vec::new();
    //let mut hash_map = HashMap::new();
    for line in lines {
        if line.starts_with("dir") || line.starts_with("$ ls") {
            continue;
        }

        let command_parts: Vec<_> = line.split_whitespace().collect();
        match command_parts[..] {
            ["$","cd", ".."] => {
                // stepping out of directory
                visited_paths.pop();
            }
            ["$","cd", name] => {
                // stepping in to directory
                //println!("Current path: {}", name);
                visited_paths.push(name);
            }
            // match on output with size and filename
            [size, _] => {
                // let full_path = affected_paths[1..affected_paths.len()].join("/");
                // println!("Current file: {} with size: {}", "/".to_owned() + &full_path + "/" + name, size);

                //println!("Current file: {} with size: {}", path.to_str().unwrap().to_owned() + "/" + name, size);

                // IK BEN DOM ~ Leclerc:(
                // for (index, _) in &mut visited_paths.into_iter().enumerate() {
                //     let copy_of_visited_paths_with_ownership_im_dumb: Vec<String> = visited_paths.into_iter().map(|path| String::from_str(path).unwrap()).collect_vec();
                //     let path = PathBuf::from_iter(&copy_of_visited_paths_with_ownership_im_dumb[..=index]);

                //     let stored_path = paths_with_sizes.entry(path).or_insert(0);
                //     *stored_path += size.parse::<u32>().unwrap();
                // }

                for index in 0..visited_paths.len() {
                    let path = PathBuf::from_iter(&visited_paths[..=index]);

                    let stored_path = paths_with_sizes.entry(path).or_insert(0);
                    *stored_path += size.parse::<u32>().unwrap();
                }
            

            }
            _ => {}
        };

        // println!("Current line: {}", line);
    }
    paths_with_sizes
}


pub fn part_one(input: &str) -> Option<u32> {
    let paths_with_sizes = input_to_sizes(input);
   
     //println!("Number of total paths {}", paths_with_sizes.len());
    // for path in paths_with_sizes.clone().into_keys() {
    //     println!("All paths: {}", path.to_str().unwrap())
    // }

    let sum_of_paths_smaller_then_100000 = paths_with_sizes.into_values()
        .filter(|size| size.to_owned() <= 100000)
        .sum::<u32>();
    
    Some(sum_of_paths_smaller_then_100000)
}


pub fn part_two(input: &str) -> Option<u32> {
    let paths_with_sizes = input_to_sizes(input);
    let root_path = PathBuf::from("/");

    let used_total = paths_with_sizes.get(&root_path).unwrap();
    let available_total = 70000000 - used_total;

    let smallest_size_to_delete = paths_with_sizes.into_values()
        .filter(|size| available_total + size >= 30000000)
        .min();
        

    smallest_size_to_delete
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
