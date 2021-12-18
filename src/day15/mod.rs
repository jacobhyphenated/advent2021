/*
Day 15: Chiton

The 2d square grid represents a risk level due to the walls being covered with Chitons.
You can move from one grid to the next up, down, left, or right (not diagonal).
For the total cost in risk starting at 0,0 (the start space risk doesn't count),
and ending in the botton right corner of the grid if you take the least risky path.

Part 1: What is the lowest risk of any path from the top left to the bottom right

Part 2: The cave is actually 5 times larger. The current grid repeats downward and to the right,
but each time it repeats the risk scores are 1 higher. If a risk score would exceed 9, it becomes 1.

*/

use std::cmp;
use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

// Create a "Risk" struct for the purposes of the priority queue
#[derive(Clone, Eq, PartialEq)]
struct Risk {
    cost: i32,
    position: (usize, usize)
}

// The priority queue in rust is a max queue, reverse the "Ord" for a min queue
impl Ord for Risk {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Risk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Part 1 & 2: Dijkstra's algorith using a priority queue
// Rust's BinaryHeap is a priority queue and uses Dijkstra's algorithm as an example in the docs
pub fn dijkstra(grid: &Vec<Vec<i32>>) -> i32 {
    // Potential risk costs all initialized to infinity (or i32::MAX)
    let mut distances: Vec<Vec<i32>> = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    let target = (grid.len() - 1, grid[0].len() - 1);

    let mut queue = BinaryHeap::new();
    
    // starting space is free
    queue.push(Risk { cost: 0, position: (0, 0)});
    distances[0][0] = 0;

    // When are priority queue is empty, the shortest distance is calculated to all points
    // pop the position with the lowest total risk cost to get there
    while let Some(current) = queue.pop() {
        // This is where we are trying to go, we're done
        if current.position == target {
            break;
        }
        let (row, col) = current.position;

        // We already found a better path to this position
        if current.cost > distances[row][col] {
            continue;
        }

        // Look at adjacent positions
        for neighbor in find_adjacent(row, col, &grid) {
            // Compute the cost to this neighbor from the current position
            let cost = distances[row][col] + grid[neighbor.0][neighbor.1];
            if cost < distances[neighbor.0][neighbor.1] {
                // if that cost is less than the known potential cost to that position
                // update the known potential costs and add to the priority queue
                distances[neighbor.0][neighbor.1] = cost;
                queue.push(Risk { cost, position: (neighbor.0, neighbor.1)});
            }
        }
    }

    return distances[target.0][target.1];
}

// Adjacent non-diagonal spaces
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

// Make the grid bigger
// there's probably a smarter modulo way to do this
pub fn expand_grid(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut expanded = grid.clone();
    for r in 0..grid.len() {
        for c in grid[0].len()..(grid[0].len() * 5) {
            let last_c = c - grid[0].len();
            let updated_val = expanded[r][last_c] + 1;
            expanded[r].push( if updated_val > 9 { 1 } else { updated_val });
        }
    }
    for r in grid.len()..(grid.len() * 5) {
        let mut row = Vec::new();
        for c in 0..expanded[0].len() {
            let last_r = r - grid.len();
            let updated_val = expanded[last_r][c] + 1;
            row.push( if updated_val > 9 { 1 } else { updated_val });
        }
        expanded.push(row);
    }

    return expanded;
}

fn parse_data(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| line.trim().chars()
            .map(|c| c.to_string().parse::<i32>().unwrap()).collect()
        )
        .collect()
}

pub fn read_grid() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("src/day15/grid.txt").expect("missing grid.txt");
    parse_data(&input)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Vec<i32>> {
        let input = "1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581";
        parse_data(input)
    }

    #[test]
    fn test_lowest_risk_path() {
        let grid = test_data();
        assert_eq!(40, dijkstra(&grid));
    }

    #[test]
    fn test_expand_grid() {
        let grid = test_data();
        let expanded = expand_grid(&grid);
        assert_eq!(2, expanded[0][10]);
        assert_eq!(3, expanded[8][11]);
        assert_eq!(1, expanded[9][14]);
        assert_eq!(9, expanded[expanded.len() -1][expanded[0].len() - 1]);
    }

    #[test]
    fn test_path_expanded() {
        let grid = test_data();
        let expanded = expand_grid(&grid);
        assert_eq!(315, dijkstra(&expanded));
    }
}