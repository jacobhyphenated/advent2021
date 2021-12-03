use std::fs;
use std::collections::HashMap;

fn most_common_digit(diagnostic: &Vec<String>, digit: usize) -> char {
    let digit_groups: HashMap<char, i32> = diagnostic.iter()
        .map(|line| line.chars().nth(digit).unwrap())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
    digit_groups.into_iter().max_by_key(|&(_, count)| count).unwrap().0
}

// 3958484
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

// pub fn life_support()

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

}
