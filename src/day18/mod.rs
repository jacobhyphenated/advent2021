/*
Day 18: Snailfish

Help snailfish with their homework. Snailfish numbers are in pairs
that contain either a number or another pair.
To add two numbers together, create a new pair of the two numbers
If a pair is 4 levels deep, it explodes
If a whole number is greater than 9, it splits

A number's magnitude is 3 * the left pair + 2 * the right pair

Part 1: given a list of numbers, add them together in order and get the magnitude
Part 2: what is the largest magnitude of any 2 combinations of numbers in the list
*/

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::fs;
use uuid::Uuid;

// Helper type to avoid writing the smart pointers everywhere
pub type SnailNumber = Rc<RefCell<SnailNumberNode>>;

// The raw node type of the number
// it can have a value, or a left/right, not both
pub struct SnailNumberNode {
    id: Uuid,
    value: Option<i32>,
    left: Option<SnailNumber>,
    right: Option<SnailNumber>,
    parent: Option<SnailNumber>
}

// custom debug display for print!
impl fmt::Debug for SnailNumberNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(val) = self.value {
            write!(f, "{}", val)
        } else {
            write!(f, "[{:?},{:?}]", self.left_unwrap().borrow(), self.right_unwrap().borrow())
        }
    }
}


// Impelment equals using a UUID for each Number Node
// Seems hacky, but #[derive(Eq, PartialEq)] has to traverse the whole number tree
// for large number graphs, this causes a call stack overflow
impl PartialEq for SnailNumberNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SnailNumberNode {}

// Constructors - use Rc<RefCell<T>> smart pointers for interior mutability.
// This pattern allows multiple references (child/parent)
// while still being able to mutate values.
// This cannot be done with lifetimes or regular pointers
impl SnailNumberNode {
    // Create a snail number from a single value
    fn from_value(value: i32) -> SnailNumber {
        Rc::new(RefCell::new(SnailNumberNode {
            id: Uuid::new_v4(),
            value: Some(value),
            left: None,
            right: None,
            parent: None
        }))
    }

    fn from_pair(left: SnailNumber, right: SnailNumber) -> SnailNumber {
        let result = Rc::new(RefCell::new(SnailNumberNode {
            id: Uuid::new_v4(),
            value: None,
            left: Some(left.clone()),
            right: Some(right.clone()),
            parent: None
        }));
        left.borrow_mut().parent = Some(result.clone());
        right.borrow_mut().parent = Some(result.clone());
        return result;
    }
}

// Snail number traversal helper methods
impl SnailNumberNode {
    /// helper method to force unrwap the left side of a pair
    /// Panics if this is a single value number
    fn left_unwrap(&self) -> SnailNumber {
        self.left.as_ref().unwrap().clone()
    }

    /// helper method to force unwrap the right side of a pair
    /// Panics if this is a single value number
    fn right_unwrap(&self) -> SnailNumber {
        self.right.as_ref().unwrap().clone()
    }

    /// Counts how deep the nested number is
    /// Note: root level counts as 1 so a number nested 4 layers deep would be 5
    fn nested(&self) -> i32 {
        if self.parent.is_none() {
            return 1;
        }
        return 1 + self.parent.as_ref().unwrap().borrow().nested()
    }

    /// Traverse the number tree to find the closes real number value
    /// to the left of the current SnailNumberNode, if one exists
    fn nearest_left(&self) -> Option<SnailNumber> {
        if let Some(p) = self.parent.as_ref() {
            let parent = p.borrow();
            if *parent.left_unwrap().borrow() == *self {
                return parent.nearest_left();
            } else {
                // traverse parent left to the right until we find a value
                let mut traverse = parent.left_unwrap();
                while traverse.borrow().value.is_none() {
                    traverse = traverse.clone().borrow().right_unwrap();
                }
                return Some(traverse.clone());
            }
        }
        None
    }

    /// Traverse the number tree to find the closest real number value
    /// to the right of the current SnailNumberNode, if one exists
    fn nearest_right(&self) -> Option<SnailNumber> {
        if let Some(p) = self.parent.as_ref() {
            let parent = p.borrow();
            if *parent.right_unwrap().borrow() == *self {
                return parent.nearest_right();
            } else {
                // traverse parent right to the left until we find a value
                let mut traverse = parent.right_unwrap();
                while traverse.borrow().value.is_none() {
                    traverse = traverse.clone().borrow().left_unwrap();
                }
                return Some(traverse.clone());
            }
        }
        None
    }

    // Calculate the magnitude for the number - recursively
    pub fn magnitude(&self) -> i32 {
        if let Some(val) = self.value {
            return val;
        }
        return 3 * self.left_unwrap().borrow().magnitude() + 2 * self.right_unwrap().borrow().magnitude();
    }
}

// Part 1: add up all the numbers
// Fold/reduce with the initial value of Option::None since no default value works for snail addition
pub fn add_all(numbers: Vec<SnailNumber>) -> SnailNumber {
    numbers.iter().fold(None, |total, rhs| {
        if let Some(lhs) = total {
            let result = add(lhs, rhs.clone());
            return Some(result);
        }
        Some(rhs.clone())
    }).unwrap()
}

// Part 2
// Normally I would reuse the Vec<SnailNumber> from part 1, but the interior mutability pattern
// we use for the number graph mutates the underlying memory references. After being added in
// part 1, the numbers are no longer the same (due to reducing).
// The same issue will happen when adding each number for part 2, so instead of creating a Vec<SnailNumber>
// we create a Vec<str> and parse out a new number each time
pub fn largest_magnitude() -> i32 {
    let input = fs::read_to_string("src/day18/numbers.txt").expect("missing numbers.txt");
    let lines: Vec<_> = input.lines().map(|l| l.trim()).collect();
    find_largest_combo_magnitude(lines)
}

// Addition is not commutative, so to brute force all combinations
// we have to add them all twice in both directions
// runs in around 3 seconds. 
fn find_largest_combo_magnitude(lines: Vec<&str>) -> i32 {
    let mut largest = 0;
    for i in 0..lines.len() {
        for j in (i+1)..lines.len() {
            let lhs = parse_line(lines[i]);
            let rhs = parse_line(lines[j]);
            let magnitude = add(lhs, rhs).borrow().magnitude();
            if magnitude > largest {
                largest = magnitude;
            }
        }
    }
    for i in (1..lines.len()).rev() {
        for j in (0..(i-1)).rev() {
            let lhs = parse_line(lines[i]);
            let rhs = parse_line(lines[j]);
            let magnitude = add(lhs, rhs).borrow().magnitude();
            if magnitude > largest {
                largest = magnitude;
            }
        }
    }
    largest
}

// Add two snail numbers
fn add(lhs: SnailNumber, rhs: SnailNumber) -> SnailNumber {
    let result = SnailNumberNode::from_pair(lhs, rhs);
    return reduce(result);
}

// Do the reducing steps in a loop until no more steps are required
fn reduce(number: SnailNumber) -> SnailNumber {
    loop {
        if explode(number.clone()) {
            continue;
        }
        if split(number.clone()) {
            continue;
        }
        break;
    }
    return number;
}

// Explode step. Traverse the numbers until we find an explosion
// return true to indicate an explosion happened somewhere
fn explode(number: SnailNumber) -> bool {
    if number.borrow().value.is_some() {
        // pairs explode, not values
        return false;
    }
    if number.borrow().nested() == 5 {
        // have to be careful about borrow and borrow_mut with the helper functions line nested() and nearest_left()
        if let Some(left_update) = number.borrow().nearest_left() {
            let new_left = left_update.borrow().value.unwrap() + number.borrow().left_unwrap().borrow().value.unwrap();
            left_update.borrow_mut().value = Some(new_left);
        }
        if let Some(right_update) = number.borrow().nearest_right() {
            let new_right = right_update.borrow().value.unwrap() + number.borrow().right_unwrap().borrow().value.unwrap();
            right_update.borrow_mut().value = Some(new_right);
        }
        let mut current = number.borrow_mut();
        current.right = None;
        current.left = None;
        current.value = Some(0);
        return true;
    }
    else {
        return explode(number.borrow().left_unwrap())
            || explode(number.borrow().right_unwrap());
    }
}

// Split number values greater than 9 into a new pair
// return true to indicate a split happened somewhere
fn split(number: SnailNumber) -> bool {
    let mut current = number.borrow_mut();
    if let Some(val) = current.value {
        if val > 9 {
            let v = val as f32 / 2.0;
            let lhs = SnailNumberNode::from_value(v.floor() as i32);
            let rhs = SnailNumberNode::from_value(v.ceil() as i32);
            
            lhs.borrow_mut().parent = Some(number.clone());
            rhs.borrow_mut().parent = Some(number.clone());
            current.value = None;
            current.left = Some(lhs);
            current.right = Some(rhs);
            return true;
        }
        return false;
    } else{
        return split(current.left_unwrap().clone()) 
            || split(current.right_unwrap().clone());
    }
}

fn parse_input(input: &str) -> Vec<SnailNumber> {
    input.lines().map(|line| parse_line(line.trim())).collect()
}

fn parse_line(input: &str) -> SnailNumber {
    let chars: Vec<char> = input.chars().collect();
    parse_snail_number(&chars[..]).0
}

fn parse_snail_number(chars: &[char]) -> (SnailNumber, usize) {
    let mut index: usize = 0;
    index += 1; // [

    let left;
    let right;
    if chars[index] == '[' {
        let (number, size) = parse_snail_number(&chars[index..]);
        left = number;
        index += size + 1;
    } else {
        let value = chars[index].to_string().parse().unwrap();
        left = SnailNumberNode::from_value(value);
        index += 1;
    }

    index += 1; // ','

    if chars[index] == '[' {
        let (number, size) = parse_snail_number(&chars[index..]);
        right = number;
        index += size + 1;
    } else {
        let value = chars[index].to_string().parse().unwrap();
        right = SnailNumberNode::from_value(value);
        index += 1;
    }

    (SnailNumberNode::from_pair(left, right), index)
}

pub fn read_input() -> Vec<SnailNumber> {
    let input = fs::read_to_string("src/day18/numbers.txt").expect("missing numbers.txt");
    parse_input(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snail_creation() {
        let sn = parse_line("[9,[8,7]]");
        assert_eq!(9, sn.borrow().left_unwrap().borrow().value.unwrap());

        let sn = parse_line("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
        assert_eq!("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]", format!("{:?}", sn.borrow()));
    }

    #[test]
    fn test_split() {
        // parser doesn't allow 2 char numbers - so for the split test, add them in after the fact
        let sn = parse_line("[[[[0,7],4],[0,[0,0]]],[1,1]]");
        sn.borrow().left_unwrap().borrow().right_unwrap().borrow().left_unwrap().borrow_mut().value = Some(15);
        sn.borrow().left_unwrap().borrow().right_unwrap().borrow().right_unwrap().borrow().right_unwrap().borrow_mut().value = Some(13);
        assert_eq!("[[[[0,7],4],[15,[0,13]]],[1,1]]", format!("{:?}", sn.borrow()));
        
        split(sn.clone());
        split(sn.clone());
        assert_eq!("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", format!("{:?}", sn.borrow()));
    }

    #[test]
    fn test_explode() {
        let sn = parse_line("[[[[[9,8],1],2],3],4]");
        explode(sn.clone());
        assert_eq!("[[[[0,9],2],3],4]", format!("{:?}", sn.borrow()));

        let sn = parse_line("[[6,[5,[4,[3,2]]]],1]");
        explode(sn.clone());
        assert_eq!("[[6,[5,[7,0]]],3]", format!("{:?}", sn.borrow()));
    }

    #[test]
    fn test_snail_addition() {
        let lhs = parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let rhs = parse_line("[1,1]");
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", format!("{:?}", add(lhs, rhs).borrow()));
    }

    #[test]
    fn test_snail_number_magnitude() {
        let sn = parse_line("[[1,2],[[3,4],5]]");
        assert_eq!(143, sn.borrow().magnitude());

        let sn = parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(3488, sn.borrow().magnitude());
    }

    #[test]
    fn test_snail_sum_magnitude() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let numbers = parse_input(input);
        let result = add_all(numbers);
        assert_eq!(4140, result.borrow().magnitude());
    }

    #[test]
    fn test_largest_combo_magnitude() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(3993, find_largest_combo_magnitude(input.lines().map(|l| l.trim()).collect()));
    }
}

