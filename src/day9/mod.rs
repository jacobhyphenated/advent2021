/*
Day 9: Smoke Basin

Volcanic smoke collects at low points in the depths.
The puzzle input is a 2d grid of depths, with 0 the lowest and 9 the highest.

Part 1: Find the low points in the depth, defined as a point where all adjacent
spaces have a higher value (do not include diagonals).
A risk level is 1 + the spaces height. Return the sum of the risk of all the low points.

Part 2: A basin is all spaces that flow downward to a single low point.
A 9 does not count as part of a basin.
Find the 3 largest basisns and return their sizes multiplied together.
*/
use std::cmp;
use std::fs;
use std::collections::HashSet;

// Part 1 - used a lot of helper methods to share code between parts
// Find the low points, add 1, then sum the values
pub fn count_low_points(grid: &Vec<Vec<i32>>) -> i32 {
    find_low_points(grid).iter()
        .map(|&(r,c)| grid[r][c] + 1)
        .sum()
}

// Start from the low points, and each low point defines a unique basin
// (we are assuming this is true, and it is true for this problem)
// Expand outward from each point to add to the basin
// Once all basins are defined, count the length and multiply the 3 highest
pub fn find_basins(grid: &Vec<Vec<i32>>) -> usize {
    let low_points = find_low_points(grid);
    let basins: Vec<HashSet<(usize, usize)>> = low_points.iter().map(|&(row,col)| {
        let mut basin = HashSet::new();
        basin.insert((row, col));

        // treat the to_expand list as a stack. Pop off the stack until empty
        let mut to_expand = expand_basin(row, col, grid, &HashSet::new());
        while let Some(next) = to_expand.pop() {
            basin.insert(next);
            to_expand.append(&mut expand_basin(next.0, next.1, grid, &basin));
        }
        basin
    }).collect();

    let mut lengths: Vec<_> = basins.iter().map(|basin| basin.len()).collect();
    lengths.sort();
    lengths.reverse();
    return lengths[0] * lengths[1] * lengths[2];
}

// Look through every space on the grid
    // find the adjacent spaces
    // if all adjacent spaces have a higher value than the current space
        // add the current space to a list as a tuple (row, col)
fn find_low_points(grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let adjacet = find_adjacent(r, c, &grid);
            if adjacet.iter().all(|&(row, col)| grid[row][col] > grid[r][c]) {
                low_points.push((r,c));
            }
        }
    }
    low_points
}

// Tricky part here is the difference in usize and i32
// usize requires a special method for subtracting
// note: nest the for loops to also get diagonals (not needed for this problem)
fn find_adjacent(row: usize, col: usize, grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    let max = grid.len() - 1;
    for r in row.checked_sub(1).unwrap_or(0)..=cmp::min(row + 1, max) {
        if r == row  {
            continue;
        }
        adjacent.push((r, col));
    }
    let max = grid[0].len() - 1;
    for c in col.checked_sub(1).unwrap_or(0)..=cmp::min(col + 1, max) {
        if c == col {
            continue;
        }
        adjacent.push((row, c));
    }
    adjacent
}

// This function takes a single space that is part of a basin
// and looks for adjacent spaces to add to the basin
// new spaces are added if
    // the value of the new space is not 9 (highest possible hight)
    // the space is not already in the basin
fn expand_basin(row: usize, col: usize, grid: &Vec<Vec<i32>>, basin: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    find_adjacent(row, col, grid).into_iter()
        .filter(|&(r, c)| grid[r][c] != 9 && !basin.contains(&(r,c)))
        .collect()
}

pub fn read_grid() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("src/day9/grid.txt").expect("missing grid.txt");
    parse_input(&input)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| line.trim().chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Vec<i32>> {
        let data = "2199943210
            3987894921
            9856789892
            8767896789
            9899965678";
        parse_input(data)
    }

    #[test]
    fn test_low_points() {
        let data = test_data();
        assert_eq!(15, count_low_points(&data));
    }

    #[test]
    fn test_basin_lengths() {
        let data = test_data();
        assert_eq!(1134, find_basins(&data));
    }
}