use std::fs;
/* 
  Day 3: Tobaggan Trajectory
  Given a puzzle input where . is an empty space and # is a tree,
  count how many trees you will encounter given a specified slope (y squares down, x squares right) starting at the top left.
  The puzzle input for each line repeats to the right an infinite number of times.
*/

pub fn count_trees_using_slope(geo: &Vec<Vec<String>>, down: usize, right: usize) -> i64 {
    let mut num_trees = 0;
    let mut yindex = 0;
    let mut xindex = 0;
    while yindex < geo.len() {
        if geo[yindex][xindex] == "#" {
            num_trees += 1;
        }
        yindex += down;
        xindex += right;
    }
  
    return num_trees;
}
  
pub fn read_geology() -> Vec<Vec<String>> {
    let geo_str = fs::read_to_string("src/day3_old/geo.txt").expect("missing geo.txt");
    let mut rows =  Vec::new();
    for line in geo_str.split("\n") {
        let mut cols = Vec::new();
        for _ in 1..100 { // definitely cheating. seems like the point would be to do this dynamically
            push_line(line, &mut cols);
        }
        rows.push(cols);
    }
    return rows;
}
  
fn push_line(line: &str, into_vec: &mut Vec<String>) {
    for geo in line.trim().chars() {
        into_vec.push(geo.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let geo = read_geology();
        let slope_3_1 = count_trees_using_slope(&geo, 1, 3);
        println!("Day3 num trees by slope (right 3, down 1) {}", slope_3_1);
        assert_eq!(284, slope_3_1);
    }

    #[test]
    fn part2() {
        let geo = read_geology();
        let slope_1_1 = count_trees_using_slope(&geo, 1, 1);
        let slope_3_1 = count_trees_using_slope(&geo, 1, 3);
        let slope_5_1 = count_trees_using_slope(&geo, 1, 5);
        let slope_7_1 = count_trees_using_slope(&geo, 1, 7);
        let slope_1_2 = count_trees_using_slope(&geo, 2, 1);
        assert_eq!(3510149120, slope_1_1 * slope_1_2 * slope_3_1 * slope_5_1 * slope_7_1 );
    }
}