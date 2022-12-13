use std::{fmt, io::{stdout, Write}, collections::VecDeque};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}, execute
};

use itertools::Itertools;

#[derive(Clone, PartialEq, Copy)]
struct Point {
    x: usize,
    y: usize,
    sighted: bool,
    chosen: bool,
}
impl Default for Point {
    fn default() -> Self { 
        Point {
            x: 0,
            y: 0,
            sighted: false,
            chosen: false,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{},{}", self.x, self.y)
    }
}

fn input_to_grid(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| {
            line.as_bytes()
            .iter()
            .map(|c| c.to_owned() as usize)
            .collect_vec()
        })
        .collect_vec()
}

fn find_init_position_on_grid(grid: &mut Vec<Vec<usize>>, char_as_byte: u8) -> Point {
    let char_to_find: usize = char_as_byte.into();
    let (x, y) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(x,y)| grid[x][y] == char_to_find)
        .unwrap();
    grid[x][y] = (char_as_byte + 24).into();
    Point { x, y, ..Default::default()}
}

fn draw_grid(grid: &Vec<Vec<usize>>, start: Point, end: Point) {
    let mut stdout = stdout();

   // stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    let (start_cursor_x, start_cursor_y) = cursor::position().unwrap();

    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .map(|(x, y)| Point { x, y, ..Default::default() })
        .for_each(|point| {
            let x = (point.y as u16) + start_cursor_y;
            let y = (point.x as u16) + start_cursor_x;

            let byte = grid[point.x][point.y];
            let display_char = String::from_utf8(vec![byte as u8]).unwrap();
            
            if point == start {
                //print!("{}", 'S');
                stdout
                    .queue(cursor::MoveTo(x, y)).unwrap()
                    .queue(style::PrintStyledContent( "S".blue().bold())).unwrap();
            } else if point == end {
                stdout
                    .queue(cursor::MoveTo(x, y)).unwrap()
                    .queue(style::PrintStyledContent( "E".blue().bold())).unwrap();
            } else {
               // print!("{}", display_char);
               let command = stdout
                    .queue(cursor::MoveTo(x, y)).unwrap();
                if point.chosen {
                    command.queue(style::PrintStyledContent( display_char.blue().bold())).unwrap();
                } else if point.sighted {
                    command.queue(style::PrintStyledContent( display_char.red().bold())).unwrap();
                } else {
                    command.queue(style::PrintStyledContent( display_char.italic())).unwrap();
                }
            }

            if point.y == grid[0].len() -1 {
                //print!("\n")
            }
        });

    print!("\n");

}

fn search_path(grid: &Vec<Vec<usize>>, start: &Point, goal: &Point) -> Option<u32> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();

    queue.push_back(((start.x, start.y), 0));
    while let Some(((x, y), len)) = queue.pop_front() {
        //println!("Currently on: {},{}", x, y);

        if (x, y) == (goal.x, goal.y) {
            return Some(len + 4);
        }
        for (dx, dy) in [(0,-1), (-1,0), (0,1), (1,0)] {
            //println!("Checking neighbour on {},{}", (x as isize + dx) as usize, (y as isize + dy) as usize);
            let (neigbour_x, neigbour_y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            //let Some(&square) = grid.get(neigbour_x).and_then(|row| row.[neigbour_y)) else { continue };
            

            // check if column fits in bounds
            if neigbour_x < grid.len() && neigbour_y < grid[neigbour_x].len() {

                //println!("Neighbour {},{} fits in grid", neigbour_x, neigbour_y);
                let square = grid[neigbour_x][neigbour_y];
                let can_climb = grid[x][y] + 1 >= square;
                let not_visited = !visited[neigbour_x][neigbour_y];

                //println!("neighbour {},{} with value {} can be climbed from current value{} : {}", neigbour_x, neigbour_y, square, grid[x][y], can_climb);
                if can_climb && not_visited  {
                    //println!("Visited neighbour on {},{}", neigbour_x, neigbour_y);
    
                    visited[neigbour_x][neigbour_y] = true;
                    queue.push_back(((neigbour_x, neigbour_y), len + 1));
                }
            
            }
            
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input_to_grid(input);
    let start = find_init_position_on_grid(&mut grid, b'S');
    let end = find_init_position_on_grid(&mut grid, b'E');

    //draw_grid(&grid, start, end);

    search_path(&grid, &start, &end)
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input_to_grid(input);
    let start = find_init_position_on_grid(&mut grid, b'S');
    let end = find_init_position_on_grid(&mut grid, b'E');

    (0..grid.len()).cartesian_product(0..grid[0].len())
        .filter(|&(x,y)| grid[x][y] == b'a'.into())
        .filter_map(|(start_x, start_y)| search_path(&grid, &Point { x: start_x, y: start_y, ..Default::default() }, &end))
        .min()

    //search_path(&grid, &start, &end)

    //None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(29));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(27));
    }
}
