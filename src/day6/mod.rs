/*
Day 6: Lanternfish

Lanternfish populations grow exponentially - each fish spawns a new fish every 7 days.
Newly born fish take an extra 2 days to spawn.
Each fish starts at a different point in their lifecycle.
Initial state: 3,4,3,1,2
After  1 day:  2,3,2,0,1
After  2 days: 1,2,1,6,0,8
After  3 days: 0,1,0,5,6,7,8

Part 1: What is the total fish population at 80 days.
Part 2: What is the total fish population at 256 days
*/
use std::fs;
use std::collections::HashMap;

/**
 * Part 1: Brute force (~350ms) 
 *      loop one day at a time, updating the counters for each fish
 *      and add new fish when required.
 */
pub fn calc_growth(fish: &Vec<i32>, days: usize) -> usize {
    let mut fish = fish.clone();
    for _ in 0..days {
        // use index for loop because mutating vector values inside a for-each is very hard
        for i in 0..fish.len() {
            fish[i] -= 1;
            if fish[i] < 0 {
                fish[i] = 6;
                fish.push(8);
            }
        }
    }
    return fish.len();
}

/**
 * Part 2: smarter way using recursion and memoization (~2ms)
 * recursively call the total_fish function - depth first traversal of fish population
 *      when a new fish is created, call total_fish for the new fish (using updated days value)
 *      use a memoization map to prevent repeat calculations
 *          Key is a tuple (fish value, days remaining)
 *          value is the total number of fish that will exist at the end
 */ 
pub fn model_growth(fish: &Vec<i32>, days: i32) -> usize {
    let mut total = 0;
    let mut memo: HashMap<(i32, i32), usize> = HashMap::new();
    for &f in fish {
        total += total_fish(f, days, &mut memo);
    }
    return total;
}

fn total_fish(initial_fish: i32, days: i32, memo: &mut HashMap<(i32, i32), usize>) -> usize {
    if let Some(total) = memo.get(&(initial_fish, days)) {
        return *total;
    }
    let mut total = 1;
    let mut days_left = days;
    let mut fish = initial_fish;
    while fish < days_left {
        // new fish created after 0, when the fish rolls back to 6
        days_left = days_left - fish - 1;
        fish = 6;
        total += total_fish(8, days_left, memo);
    }
    memo.insert((initial_fish, days), total);
    return total;
}

pub fn read_input() -> Vec<i32> {
    let fish = fs::read_to_string("src/day6/fish.txt").expect("missing fish.txt");
    fish.split(",").map(|f| f.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_growth() {
        let init = vec![3,4,3,1,2];
        assert_eq!(26, calc_growth(&init, 18));
        assert_eq!(5934, calc_growth(&init, 80));
    }

    #[test]
    fn test_model_growth() {
        let init = vec![3,4,3,1,2];
        assert_eq!(26984457539, model_growth(&init, 256));
    }

}