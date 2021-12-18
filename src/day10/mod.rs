
use std::collections::HashMap;
use std::fs;

// Part 1 & Part 2
// Both parts ended up being so similar, that I combined both into one method
// Returns a tuple - first value is part 1, second is part 2
// Keeps a LIFO stack of the next closing character.
// Each time an open character is encountered, the corresponding close character is added to the stack.
// if the next character is a closing character but not the next value on the stack
//      then this is an illegal line - score appropriately
// If we traverse the line without an illegal closing character
//      then what is left in the stack are the required closing characters to complete the line
//      score those closing characters appropriately
pub fn syntax_score(lines: &Vec<String>) -> (i32, i64) {
    let closing_map: HashMap<char, char> = vec!['(', '[', '{', '<'].into_iter()
        .zip(vec![')', ']', '}', '>'].into_iter())
        .collect();

    let invalid_scores: HashMap<char, i32> = vec![')', ']', '}', '>'].into_iter()
        .zip(vec![3, 57, 1197, 25137].into_iter())
        .collect();

    let incomplete_scores: HashMap<char, i64> = vec![')', ']', '}', '>'].into_iter()
        .zip(1..=4)
        .collect();
    
    let mut invalid_score = 0;
    let mut incomplete: Vec<i64> = vec![];
    for line in lines {
        let mut next_closing_stack: Vec<&char> = vec![];
        let mut invalid = false;
        for next_char in line.chars() {
            // if "next_char" is an open character, add the corresponding close char to the stack
            if let Some(close_char) = closing_map.get(&next_char) {
                next_closing_stack.push(close_char);
            } else {
                // it's not an open char, so it must be a close, pop the next off the stack
                let expected = next_closing_stack.pop().unwrap_or(&'-');
                // if the popped expected close char is not the next_char, this is an illegal line
                if expected != &next_char {
                    invalid_score += invalid_scores.get(&next_char).unwrap();
                    invalid = true;
                    break;
                }
            }
        }
        if !invalid {
            // The line was not invalid, so it must be incomplete.
            // reverse the stack to get the appropriate order of the required close characters
            next_closing_stack.reverse();
            let final_score = next_closing_stack.into_iter().fold(0, |score, c|{
                score * 5 + incomplete_scores.get(c).unwrap()
            });
            incomplete.push(final_score);
        }
    }

    incomplete.sort();
    return (invalid_score, incomplete[incomplete.len() / 2]);
}

pub fn read_lines() -> Vec<String> {
    let lines = fs::read_to_string("src/day10/lines.txt").expect("missing lines.txt");
    lines.lines().map(|line| line.trim().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<String> {
        let data = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";
        data.lines().map(|line| line.trim().to_string()).collect()
    }

    #[test]
    fn test_score_illegal_chars() {
        let lines = test_data();
        assert_eq!((26397,288957), syntax_score(&lines));
    }
}

