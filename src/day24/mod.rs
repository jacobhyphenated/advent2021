/*
Day 24: Arithmatic Logic Unit

The ALU is a program language with instructions:
inp, add, mul, div, mod, eql

The puzzle input is an ALU program for validating a 14 digit model number.
Each digit will be 1 of the 14 "inp" input instructions, starting on the left.
If the number is valid, the ALU will exit with a value of 0 in the z register.

Part 1: what is the largest valid model number?
Part 2: what is the smallest valid model number?
*/

use std::fs;

// Track each instruction with the command (add), the target register, and the optional value
pub struct Instruction {
    command: String,
    target: String,
    operand: Option<String>
}

// ALU struct mutates with each instruction executed
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    input: Box<dyn Iterator<Item=i64>>
}

impl ALU {
    // define the ALU by the input - represented by an iterator
    fn new(input: Box<dyn Iterator<Item=i64>>) -> ALU {
        ALU {
            w: 0, x: 0, y: 0, z: 0, input
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        let target = self.dimension(&instruction.target);
        let result = match &instruction.command[..] {
            "inp" => self.input.next().unwrap(),
            "add" => target + self.dimension(&instruction.operand.as_ref().unwrap()),
            "mul" => target * self.dimension(&instruction.operand.as_ref().unwrap()),
            "div" => target / self.dimension(&instruction.operand.as_ref().unwrap()),
            "mod" => target % self.dimension(&instruction.operand.as_ref().unwrap()),
            "eql" => if target == self.dimension(&instruction.operand.as_ref().unwrap()) { 1 } else { 0 },
            _ => panic!("Invalid command: {}", instruction.command) 
        };
        match &instruction.target[..] {
            "w" => self.w = result,
            "x" => self.x = result,
            "y" => self.y = result,
            "z" => self.z = result,
            _ => panic!("Invalid target {}", instruction.target)
        };
    }

    fn dimension(&self, value: &str) -> i64 {
        match value {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            _ => value.parse().unwrap()
        }
    }
}

/*
The code here runs, and you can use it to double check if a model number is valid,
but this is solved via pen and paper.

Logic is approached like this:
There are 14 steps that are nearly identical.
The biggest differences are some steps have `div z 26` vs `div z 1`
All steps have this sequence:
    add x z
    mod x 26
    add x <n1> // line 6 of each step
    eql x w
    eql x 0
To avoid adding extra y values to the z register, we need to get x == w (the input)
x will be z % 26 - and parts of z are multipled previously by 26, allowing us to truncate that part

For steps where it is not possible to get x == w:
    z = 26z + w + <n2>
    where <n2> comes from `add y <n2>` on line 16 of each step

To start, step 1 has <n2> == 4.
after step 1, z = i1 + 4 (where i1 is the input digit w for step 1)

step 2 has 
    <n1> 15
    <n2> 11
    z = 26z1 + i2 + 11

after step 3, z = 26z2 + i3 + 7

step 4 is our first div z 26 step.
    <n1> -14
    (26z2 + i3 + 7) % 26 = i3 + 7
    for i4 such that i4 = i3+7
    z = (26z2 + (i3 + 7)) / 26 = z2 = 26z1 + i2 + 11
        Note that (i3 + 7) / 26 = 0 as i3 + 7 is always less than 26 and we truncate the division

After step 4 we can create two logical rules:
    i3 must be 8 or 9
    i4 must be i3 minus 7

Proceed for the remaining steps creating logical rules like this as we go
This greatly restricts the number of valid modal numbers and the min/max can be
figured out by applying largest/smallest values allowed within the rules for each digit.

Final rules are as follows:
    i1 must be 9
    i2 must be 1 or 2
    i3 must be 8 or 9
    i4 must be i3 minus 7
    i5 must be between 1 and 8 inclusive
    i6 must be i5 plus 1
    i7 must be 1
    i8 must be between 1 and 4 inclusive
    i9 must be i8 + 5
    i11 must equal i10
    i12 must be 9
    i13 must be i2 + 7
    i14 must be 1
*/
pub fn validate_modal_number(modal_number: &str, instructions: &Vec<Instruction>) -> bool {
    let input: Vec<i64> = modal_number.chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
    let mut alu = ALU::new(Box::new(input.into_iter()));
    execute_instruction(&mut alu, &instructions);
    return alu.z == 0;
}

fn execute_instruction(alu: &mut ALU, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        alu.execute(instruction);
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| {
            let parts:Vec<_> = line.trim().split(" ").collect();
            Instruction{
                command: parts[0].to_string(),
                target: parts[1].to_string(),
                operand: parts.get(2).map(|val| val.to_string())
            }
        })
        .collect()
}

pub fn read_instructions() -> Vec<Instruction> {
    let input = fs::read_to_string("src/day24/instructions.txt").expect("missing instructions.txt");
    parse_instructions(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_alu_instructions() {
        let input = "inp z
            inp x
            mul z 3
            eql z x";
        let instructions = parse_instructions(input);

        let mut alu = ALU::new(Box::new(vec![22,66].into_iter()));
        execute_instruction(&mut alu, &instructions);
        assert_eq!(1, alu.z);

        let mut alu = ALU::new(Box::new(vec![22,51].into_iter()));
        execute_instruction(&mut alu, &instructions);
        assert_eq!(0, alu.z);
    }

    #[test]
    fn test_execute_alu_binary_digits() {
        let input = "inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2";
        let instructions = parse_instructions(input);

        let mut alu = ALU::new(Box::new(vec![5].into_iter()));
        execute_instruction(&mut alu, &instructions);
        assert_eq!(1, alu.z);
        assert_eq!(0, alu.y);
        assert_eq!(1, alu.x);
        assert_eq!(0, alu.w);
    }


    #[test]
    fn test_model_first_round_only() {
        let input = "inp w
            mul x 0
            add x z
            mod x 26
            div z 1
            add x 12
            eql x w
            eql x 0
            mul y 0
            add y 25
            mul y x
            add y 1
            mul z y
            mul y 0
            add y w
            add y 4
            mul y x
            add z y";
        let instructions = parse_instructions(input);
        for i in 1..=9 {
            let mut alu = ALU::new(Box::new(vec![i].into_iter()));
            execute_instruction(&mut alu, &instructions);
            println!("input = {}, z = {}", i, alu.z);
        }
    }

    #[test]
    fn test_verify_model_single() {
        let instructions = read_instructions();
        let input: Vec<i64> = "91811211611981".chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
        let mut alu = ALU::new(Box::new(input.into_iter()));
        execute_instruction(&mut alu, &instructions);
        println!("z = {}", alu.z);
    }

}