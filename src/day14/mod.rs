/*
Day 14: Extended Polymerization

Build a polyer from a polymer template. The pair insertion rules
show what element gets inserted between elements on each step. For example:
CH -> B
HH -> N
CB -> H
BH -> H
HN -> C
NH -> C

If you start with CHH
after one step:   CBHNH
After two steps:  CHBHHCNCH

Part 1: Run 10 steps. Count the number of times the most common element
appears and subtract the number of times the least common element appears

Part 2: Do the same as part 1 but for 40 steps
*/
use std::collections::HashMap;
use std::fs;

// Part 1: brute force
// resolve the next polymer after each step
// after all steps are complete, count up each individual character
pub fn common_polymers(template: &str, pair_insertion: &HashMap<String, char>, steps: i32) -> i64 {
    let mut polymer: Vec<char> = template.chars().collect();
    for _ in 0..steps {
        polymer = next_polymer(&polymer, pair_insertion);
    }
    let element_count: HashMap<char, i64> = polymer.into_iter().fold(HashMap::new(), |mut count_map, c|{
        *count_map.entry(c).or_insert(0) += 1;
        count_map
    });
    
    return element_count.values().max().unwrap() - element_count.values().min().unwrap();
}

// Helper method for part 1
// takes the starting position, inserts the pair insertion character between each group of two characters
// track the polymer as a vector of characters
fn next_polymer(start: &Vec<char>, pair_insertion: &HashMap<String, char>) -> Vec<char> {
    let mut polymer = start.windows(2).fold(vec![], |mut p, char_pair|{
        let key: String = char_pair.iter().collect();
        p.push(char_pair[0]);
        p.push(pair_insertion[&key]);
        p
    });
    polymer.push(*start.last().unwrap());
    return polymer;
}

// Part 2 - Make it not O(M * 2^n)
// Rather than tracking characters in order each step, track the pairs
// rather than CH -> B, use CH -> [CB, BH]
// It actually doesn't matter what order the polymer pairs appear in, so just count the number of unique pairs
pub fn polymers_as_pairs(template: &str, pair_insertion: &HashMap<String, char>, steps: i32) -> i64 {

    // First, reframe our pair insertion map to map from one pair to two polymer pairs
    let pair_map: HashMap<String, Vec<String>> = pair_insertion.iter()
        .map(|(k,v)|{
            let mut vec = vec![];
            let mut chrs = k.chars();
            vec.push(format!("{}{}", chrs.next().unwrap(), v));
            vec.push(format!("{}{}", v, chrs.next().unwrap()));
            (k.to_string(), vec)
        }).collect();
    
    // Turn our template polymer into string pairs, then count them
    let chars: Vec<char> = template.chars().collect();
    let mut pair_count: HashMap<String, i64> = chars.windows(2).map(|cs| {
        let mut i = cs.iter();
        format!("{}{}", i.next().unwrap(), i.next().unwrap())
    }).fold(HashMap::new(), |mut map, pair| {
        *map.entry(pair).or_insert(0) += 1;
        map
    });

    for _ in 0..steps {
        // We start with our existing count of pairs
        pair_count = pair_count.into_iter().fold(HashMap::new(), |mut map, (pair, count)|{
            // Turn each pair into two new pairs
            let new_pairs = pair_map.get(&pair).unwrap();
            for p in new_pairs {
                // Each new pair gets the original pair's count added to that pair's new total
                // Ex. if there were 14 CH, then we add 14 to CB and 14 to BH
                *map.entry(p.to_string()).or_insert(0) += count;
            }
            map
        });
    }
    let mut element_count: HashMap<char, i64> = pair_count.into_iter().fold(HashMap::new(), |mut map, (pair, count)| {
        // count the first character only
        // the last character is always the first character of another pair
        let c = pair.chars().next().unwrap();
        *map.entry(c).or_insert(0) += count;
        map
    });
    // except the very last character
    *element_count.entry(template.chars().last().unwrap()).or_insert(0) += 1;

    return element_count.values().max().unwrap() - element_count.values().min().unwrap();
}

fn parse_pair_map(input: &str) -> HashMap<String, char> {
    input.lines().fold(HashMap::new(), |mut map, pair| {
        let pair: Vec<_> = pair.trim().split(" -> ").collect();
        map.insert(pair[0].to_string(), pair[1].chars().next().unwrap());
        map
    })
}

pub fn read_polymer_data() -> (String, HashMap<String, char>) {
    let input = fs::read_to_string("src/day14/pairs.txt").expect("missing pairs.txt");
    let template = "PHVCVBFHCVPFKBNHKNBO".to_string();
    (template, parse_pair_map(&input))
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_pair_insertion() -> HashMap<String, char> {
        let input = "CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C";
        parse_pair_map(input)
    }

    #[test]
    fn test_common_elements() {
        let init = "NNCB";
        let pair_insertion = get_pair_insertion();
        assert_eq!(1588, common_polymers(init, &pair_insertion, 10));
    }

    #[test]
    fn test_pair_group() {
        let init = "NNCB";
        let pair_insertion = get_pair_insertion();
        assert_eq!(1588, polymers_as_pairs(init, &pair_insertion, 10));
        assert_eq!(2188189693529, polymers_as_pairs(init, &pair_insertion, 40));
    }   
}
