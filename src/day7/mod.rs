/*
Day 7: The Treachery of Whales

To escape a giant whale, a bunch of crabs in submarines have to get to the same horizontal position.
These submarines have limited gas, so we have to find the position that takes the least amount of gas
for all the crab submarines to reach.

Part 1: gas is computed linearly: a crab at position 5 requires 3 gas to reach position 2.

Part 2: gas is computed by adding an additional unit per horizontal space moved.
Moving from 5 -> 4 = 1, from 5 -> 3 = 1 + 2, etc. So Moving from position 5 to position 2 requires 6 gas. 
*/

use std::cmp;
use std::fs;

fn calc_gas(subs: &Vec<i32>, position: i32) -> i32 {
    subs.iter().fold(0, |acc, sub| acc + (sub - position).abs())
}

// 1+2+3+4..n == (n * (n+1)) / 2
fn calc_gas_exp(subs: &Vec<i32>, position: i32) -> i32 {
    subs.iter().fold(0, |acc, sub| {
        let n = (sub - position).abs();
        acc + (n * (n + 1)) / 2
    })
}

/**
 * Part 1. The cheapest position in terms of gas is the median position.
 * I don't have a proof for why that's true. I reason it out as follows:
 *      Outliers don't matter, take an example of [10000, 1, 0].
 *      position 1 is best at 10000
 *      Moving closer to the outlier reduces the cost for the outlier,
 *      but makes it more expensive for the other 2 at a tradeoff of 2 to 1.
 */ 
pub fn linear_gas(subs: &Vec<i32>) -> i32 {
    let mut sorted_subs = subs.clone();
    sorted_subs.sort();
    let median = sorted_subs.len() / 2;
    return cmp::min(calc_gas(&sorted_subs, sorted_subs[median]), calc_gas(&sorted_subs, sorted_subs[median + 1]));
}

/**
 * Prt 2. The cheapest position in terms of gas is the average position.
 * I don't have a proof for why that's true. I reason it out as follows:
 *      Outliers now matter, because moving 1 additional space costs more for the outliers
 *      The average balances out the large cost of moving outliers with
 *      additional (less expensive) movement from the values close to median
 */ 
pub fn exponential_gas(subs: &Vec<i32>) -> i32 {
    let mut sorted_subs = subs.clone();
    sorted_subs.sort();
    let average = sorted_subs.iter().sum::<i32>() / sorted_subs.len() as i32;
    return cmp::min(calc_gas_exp(&sorted_subs, average), calc_gas_exp(&sorted_subs, average + 1));
}

pub fn read_input() -> Vec<i32> {
    let input = fs::read_to_string("src/day7/subs.txt").expect("missing subs.txt");
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_calc() {
        let subs = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(37, calc_gas(&subs, 2));
        assert_eq!(41, calc_gas(&subs, 1));
        assert_eq!(71, calc_gas(&subs, 10));
    }

    #[test]
    fn test_gas_exp() {
        let subs = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(206, calc_gas_exp(&subs, 2));
        assert_eq!(168, calc_gas_exp(&subs, 5));
    }

    #[test]
    fn test_cheapest_gas() {
        let subs = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(37, linear_gas(&subs));
    }

    #[test]
    fn test_cheapest_exp() {
        let subs = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(168, exponential_gas(&subs));
    }
}