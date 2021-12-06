/*
Day 3: Binary Diagnostic

Input is a list of numbers in binary code

Part 1: For each bit of all of the numbers, find the most common bit (1 or 0)
For the value of gamma, set the value of that bit to the most common.
For the value of epsilon, set the value for that bit to be the least common.
Retun gamma times epsilon.

Part2: For each bit starting on the left, find the most common, then keep only the
binary numbers that have that most common value as the bit for that place in the number.
Once the list of numbers is down to 1, that is the oxygen value. In case of a tie, use 1.
Use the same process except finding the least common bit for the co2 value. In case of a tie, use 0.
Return oxygen times co2.
*/

use std::fs;
use std::collections::HashMap;

fn most_common_digit(diagnostic: &Vec<String>, digit: usize) -> char {
    let digit_groups: HashMap<char, i32> = diagnostic.iter()
        .map(|line| line.chars().nth(digit).unwrap())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
    let one_count = digit_groups.get(&'1').unwrap();
    let zero_count = digit_groups.get(&'0').unwrap();
    if one_count >= zero_count {
        return '1';
    } else {
        return '0';
    }
}

pub fn power(diagnostic: &Vec<String>) -> i32 {
    let length = diagnostic[0].len(); 
    let mut epsilon: Vec<char> = vec![];
    let mut gamma: Vec<char> = vec![];
    for place in 0..length {
        let most_common = most_common_digit(diagnostic, place);
        if most_common == '1' {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma = i32::from_str_radix(&gamma.into_iter().collect::<String>()[..], 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon.into_iter().collect::<String>()[..], 2).unwrap();
    return gamma * epsilon;
}

pub fn life_support(diagnostic: &Vec<String>) -> i32 {
    let mut oxygen = diagnostic.clone();
    let mut place = 0;
    while oxygen.len() > 1 {
        let most_common = most_common_digit(&oxygen, place);
        oxygen = oxygen.into_iter().filter(|line| line.chars().nth(place).unwrap() == most_common).collect();
        place += 1;
    }
    let oxygen = i32::from_str_radix(&oxygen[0][..], 2).unwrap();

    let mut co2 = diagnostic.clone();
    let mut place = 0;
    while co2.len() > 1 {
        let least_common = match most_common_digit(&co2, place) {
            '1' => '0',
            _ => '1'
        };
        co2 = co2.into_iter().filter(|line| line.chars().nth(place).unwrap() == least_common).collect();
        place += 1;
    }
    let co2 = i32::from_str_radix(&co2[0][..], 2).unwrap();

    return co2 * oxygen;
}

pub fn read_diagnostic() -> Vec<String> {
    let file = fs::read_to_string("src/day3/diag.txt").expect("file diag.txt not found");
    file.lines().map(|line| line.trim().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<String> {
        let test = "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010";
        test.lines().map(|line| line.trim().to_string()).collect()
    }

    #[test]
    fn test_power() {
        let diag = get_test_data();
        assert_eq!(198, power(&diag));
    }

    #[test]
    fn test_life_support() {
        let diag = get_test_data();
        assert_eq!(230, life_support(&diag));
    }

}
