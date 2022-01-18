/*
Day 25: Sea Cucumber

Sea cucumbers are either left ">" or down "v".
For each step, a left sea cucumber moves left if the space left of it is empty.
A down sea cucumber moves down if the space below it is empty.
Both left and down wrap around to 0 when the reach the left/bottom edge.

All left cucumbers evaluate at the same time, then move.
Then all down cucumbers evaluate at the same time, then move.

Part 1: What is the number of the first step when no sea cucumbers move?
 */
use std::fs;

#[derive(Clone, PartialEq, Debug)]
pub enum Location {
    Left, Down, Empty
}

impl Location {
    fn from_char(c: &char) -> Location {
        match c {
            'v' => Location::Down,
            '>' => Location::Left,
            '.' => Location::Empty,
            _ => panic!("Invalid Location char {}", c)
        }
    } 
}

// Part 1: loop until there is no movement
pub fn find_stable_step(grid: &Vec<Vec<Location>>) -> usize {
    let mut grid = grid.clone();
    let mut step = 1;
    while do_step(&mut grid) != 0 {
        step += 1;
        if step % 10 == 0 {
            println!("step {}", step);
        }
    }
    return step;
}

// Evaluates the grid at the end of the step.
// This mutates the grid in place
// Returns the number of sea cucumbers that moved
fn do_step(grid: &mut Vec<Vec<Location>>) -> usize {
    
    // First evaluate the left, find all the left facing cucumbers that will move this step
    let mut left_changes: Vec<(usize, usize)> = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == Location::Left && grid[row][next_left(col, &grid)] == Location::Empty {
                left_changes.push((row, col));
            }
        }
    }
    // move all the left facing cucumbers that are eligible
    for (r,c) in left_changes.iter() {
        let left = next_left(*c, &grid);
        grid[*r][*c] = Location::Empty;
        grid[*r][left] = Location::Left;
    }

    // Now evaluate the down sea cucumbers
    let mut down_changes: Vec<(usize, usize)> = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == Location::Down && grid[next_down(row, &grid)][col] == Location::Empty {
                down_changes.push((row, col));
            }
        }
    }
    // move down sea cucumbers that are eligible
    for (r,c) in down_changes.iter() {
        let down = next_down(*r, &grid);
        grid[*r][*c] = Location::Empty;
        grid[down][*c] = Location::Down;
    }
    left_changes.len() + down_changes.len()
}

fn next_left(col: usize, grid: &Vec<Vec<Location>>) -> usize {
    let next = col + 1;
    if grid[0].len() <= next {
        return 0;
    }
    return next;
}

fn next_down(row: usize, grid: &Vec<Vec<Location>>) -> usize {
    let next = row + 1;
    if grid.len() <= next {
        return 0;
    }
    return next;
}


fn parse_input(input: &str) -> Vec<Vec<Location>> {
    input.lines()
        .map(|line| line.trim().chars()
            .map(|c| Location::from_char(&c))
            .collect()
        )
        .collect()
}

pub fn read_grid() -> Vec<Vec<Location>> {
    let input = fs::read_to_string("src/day25/grid.txt").expect("missing grid.txt");
    parse_input(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_location_step() {
        let input = "...>...
            .......
            ......>
            v.....>
            ......>
            .......
            ..vvv..";
        let mut grid = parse_input(input);
        let moves = do_step(&mut grid);
        assert_eq!(5, moves);
        assert_eq!(Location::Down, grid[0][2]);
        assert_eq!(Location::Down, grid[0][3]);
        assert_eq!(Location::Left, grid[0][4]);
    }

    #[test]
    fn test_find_stable_step() {
        let input = "v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>";
        let grid = parse_input(input);
        assert_eq!(58, find_stable_step(&grid));
    }
}