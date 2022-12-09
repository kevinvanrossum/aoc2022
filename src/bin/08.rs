use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let grid = lines.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()).collect_vec();
    // let mut visible = 0;

    // for row_index in 0..grid.len() {
    //     let row = grid.get(row_index).unwrap();

    //     for column_index in 0..row.len() {
    //         let tree_to_check = row.get(column_index);

    //         let visible_left = row[0..column_index].iter().max() < tree_to_check;
    //         let visible_top = grid[0..row_index].iter().map(|row| row.get(column_index).unwrap()).max() < tree_to_check;
    //         let visible_right = row[column_index+1..row.len()].iter().max() < tree_to_check;
    //         let visible_bottom = grid[row_index+1..grid.len()].iter().map(|row| row.get(column_index).unwrap()).max() < tree_to_check;

    //         if visible_left || visible_right || visible_top || visible_bottom {
    //             // println!("visible tree: {} on row: {} column:: {}", tree_to_check.unwrap(), row_index, column_index);
    //             visible+=1;
    //         }
    //     }
    // }

    // After learning about cartesian product and borrowing :)

    let visible_from_outside = (grid.len() - 1) * 4; 
    // Skipping outside border these are alawys visible
    let visible_in_inner_grid = (1..grid.len() - 1) 
        .cartesian_product(1..grid.len() - 1)
        .map(|(y, x)| {
            let current_tree = grid[y][x];
            get_neigbor_rows(&grid, x, y)
                .iter()
                .map(|direction| {
                    direction.iter().all(|neighour_tree| *neighour_tree < current_tree)
                })
                .any(|visible| visible)
        })
        .filter(|visibile| *visibile)
        .count();

    Some((visible_from_outside + visible_in_inner_grid) as u32)

   // Some(visible)
}

fn get_neigbor_rows(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();

    let (up, down) = column.split_at(y);
    let (left, right) = row.split_at(x);

    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    [up, down, left, right]
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()).collect_vec();
    
    let scenic_scores = (1..grid.len() - 1) 
        .cartesian_product(1..grid.len() - 1)
        .map(|(y, x)| {
            let current_tree = grid[y][x];
            get_neigbor_rows(&grid, x, y)
            .iter()
            .map(|direction| {
                direction
                    .iter()
                    .position(|neighbour_tree| *neighbour_tree >= current_tree)
                    // add 1 to the neighbour location for seeing distance
                    .map(|blocking_neighour| blocking_neighour + 1)
                    .unwrap_or_else(|| direction.len()) as u32
            })
            .product::<u32>()
        });

        Some(scenic_scores.max().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
