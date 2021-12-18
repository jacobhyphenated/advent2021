/*
Day 8: Seven Segment Search

The seven segment displays (number segments) are out of order.
Normally a 7 segment number is defined by:
 aaaa
b    c
b    c
 dddd
e    f
e    f
 gggg

Normally a 7 would be defined as "acf", but the segments are out of order and random.
Given an input like: 
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
The first values are observed, the values after the | are what we need to decode.

Part 1: We can determine some digits by length (1 = 2, 7 = 3, 4 = 4, 8 = 7).
Return the number of times the digits 1, 4, 7, and 8 appear.

Part 2: Figure out the random order for each line of the input.
The output values are to the right side of the |
so "cdfeb fcadb cdfeb cdbaf" would be 5353
Add up all the outupt numbers
*/

use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SevenSegmentData {
    training: Vec<String>,
    output: Vec<String>
}

// Part 1
pub fn count_known_values(data: &Vec<SevenSegmentData>) -> usize {
    data.iter()
        .flat_map(|d| d.output.iter())
        .filter(|digit| digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7 )
        .count()
}

// Part 2: mostly brute force (350ms). See inline comments
// future note: a better way to do this is to define each number as sub and super sets:
//      for example, 3 is a superset of 7 with length 5
//      9 is a superset of 3 with length 6 (etc)
pub fn decode_values(segment_data: &Vec<SevenSegmentData>) -> i32 {
    // Define the valid seven segment rules
    let mut digit_map: HashMap<&str, &str> = HashMap::new();
    digit_map.insert("abcefg", "0");
    digit_map.insert("cf", "1");
    digit_map.insert("acdeg", "2");
    digit_map.insert("acdfg", "3");
    digit_map.insert("bcdf", "4");
    digit_map.insert("abdfg", "5");
    digit_map.insert("abdefg", "6");
    digit_map.insert("acf", "7");
    digit_map.insert("abcdefg", "8");
    digit_map.insert("abcdfg", "9");

    // One at a time, with a counter. So far so good.
    let mut result = 0;
    for data in segment_data {

        // sort the scrambled codes by length - hit the easy ones first.
        let mut training_data: Vec<&String> = data.training.iter().chain(data.output.iter()).collect();
        training_data.sort_by_key(|s| s.len());
        
        // Try to map the scrambled digit to a set of possible real positions it could occupy
        // start with all possibilities and narrow down as we go
        let mut decoder: HashMap<char, HashSet<char>> = "abcdefg".chars()
            .fold(HashMap::new(), |mut map, c| {
                map.insert(c, "abcdefg".chars().collect());
                map
            });
        
        for training in &training_data {
            // Get all the digits that our scrambled character might map to, based on the length of the value.
            // Do a set intersection to narrow down the potential values for each scrambled digit
            let possible_digits: HashSet<char> = digit_map.keys()
                .filter(|key| key.len() == training.len())
                .flat_map(|digit| digit.chars())
                .collect();
            for random_char in training.chars() {
                let v = decoder.get(&random_char).unwrap();
                // set intersection is an interator on references
                // annoyingly, have to dereferenc in order to re-assign the set
                *decoder.get_mut(&random_char).unwrap() = v.intersection(&possible_digits).map(|&c| c).collect();
            }
        }

        // Attempts to do smarter rules based logical deductions ended in failure and frustration
        // Let's brute force this bad boy
        let possible_solutions = Vec::from_iter(decoder.get(&'a').unwrap().iter().map(|c| c.to_string()));
        let possible_solutions: Vec<String> = "bcdefg".chars().fold(possible_solutions, |sol, c| {
            let vals = decoder.get(&c).unwrap();
            if vals.len() == 1 {
                sol.into_iter().map(|val| format!("{}{}", val, c)).collect()
            } else {
                vals.iter().map(|v|
                    sol.iter()
                        .filter(|prev| !prev.contains(&v.to_string()))
                        .map(|prev| format!("{}{}", prev, v))
                        .collect::<Vec<_>>()
                ).flat_map(|nest| nest.into_iter()).collect()
            }
        });
        // Above is a lot of mapping and flat mapping to get a complete enumeration of possible solutions
        // Now we try out each solution until we find one that doesn't violate the seven segment rules
        for solution in possible_solutions {
            if solution.len() != 7 {
                continue;
            }
            // this line is maybe the only cool thing I did for this entire problem
            let decoder: HashMap<char, char> = ('a'..='g').zip(solution.chars()).collect();
            let mut valid_solution = true;
            for test_value in &training_data {
                let mut decoded: Vec<&char> = test_value.chars()
                    .map(|c| decoder.get(&c).unwrap())
                    .collect();
                decoded.sort();
                let decoded: String = decoded.into_iter().collect();
                if !digit_map.contains_key(&decoded[..]) {
                    // Decodes to something that's not a digit - try the next solution
                    valid_solution = false;
                    break;
                }
            }
            if valid_solution {
                // We did it! Lets spend 12 lines turning a string into a number
                let number: String = data.output.iter().map(|value| {
                        let mut decoded: Vec<&char> = value.chars()
                            .map(|c| decoder.get(&c).unwrap())
                            .collect();
                        decoded.sort();
                        let decoded: String = decoded.into_iter().collect();
                        *digit_map.get(&decoded[..]).unwrap()
                    })
                    .collect();
                result += number.parse::<i32>().unwrap();
                break;
            }
        }
    }

    return result;
}

pub fn read_data() -> Vec<SevenSegmentData> {
    let data = fs::read_to_string("src/day8/segments.txt").expect("missing segments.txt");
    parse_data(&data)
}

fn parse_data(data: &str) -> Vec<SevenSegmentData> {
    data.lines().map(|line| {
        let parts: Vec<Vec<String>> = line.split(" | ")
            .map(|part| part.trim().split_whitespace().map(|val| val.to_string()).collect::<Vec<_>>())
            .collect();
        // can't just do (parts[0], parts[1]) - need to move the memory rather than borrow
        let mut iter = parts.into_iter();
        SevenSegmentData { training: iter.next().unwrap(), output: iter.next().unwrap() }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<SevenSegmentData> {
        let data = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        parse_data(data)
    }

    #[test]
    fn test_known() {
        let data = test_data();
        assert_eq!(26, count_known_values(&data));
    }

    #[test]
    fn test_sum_decoded() {
        let data = test_data();
        assert_eq!(61229, decode_values(&data));
    }
}
