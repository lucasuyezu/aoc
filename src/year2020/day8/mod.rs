use std::collections::HashSet;

#[derive(Debug, Clone)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: OpCode,
    argument: isize,
}

impl Instruction {
    fn execute(&self, acc: isize) -> isize {
        match self.opcode {
            OpCode::Acc => acc + self.argument,
            _ => acc,
        }
    }

    fn next_instruction_id(&self, current_instruction_id: usize) -> usize {
        match self.opcode {
            OpCode::Jmp => (current_instruction_id as isize + self.argument) as usize,
            _ => current_instruction_id + 1,
        }
    }
}

fn build_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();

    for line in input.lines() {
        let opcode = match &line[0..3] {
            "nop" => OpCode::Nop,
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            _ => panic!("Invalid instruction!"),
        };

        let argument_str = &line[4..];
        let argument = argument_str.parse::<isize>().unwrap();

        instructions.push(Instruction {
            opcode,
            argument,
        });
    }

    instructions
}

fn execute_program(instructions: &[Instruction]) -> Result<isize, isize> {
    let mut current_instruction_id = 0;
    let mut acc = 0;
    let mut instructions_set = HashSet::<usize>::new();

    while current_instruction_id < instructions.len() {
        if instructions_set.contains(&current_instruction_id) {
            return Err(acc);
        }

        instructions_set.insert(current_instruction_id);

        let current_instruction = &instructions[current_instruction_id];

        // dbg!(&current_instruction_id);
        // dbg!(&acc);
        // dbg!(&current_instruction);

        acc = current_instruction.execute(acc);
        current_instruction_id = current_instruction.next_instruction_id(current_instruction_id);
    }

    Ok(acc)
}

pub fn solve_part_1(input: &str) -> isize {
    let instructions = build_instructions(input);

    if let Err(acc) = execute_program(&instructions) {
        return acc;
    }

    panic!("Part 1 never finishes!")
}

pub fn solve_part_2(input: &str) -> isize {
    let instructions = build_instructions(input);

    for i in 0..instructions.len() {
        let mut program = instructions.clone();
        let mut instruction = &mut program[i];

        if let OpCode::Acc = instruction.opcode {
            continue;
        }

        // replace instruction id with the other one
        instruction.opcode = match instructions[i].opcode {
            OpCode::Jmp => OpCode::Nop,
            OpCode::Nop => OpCode::Jmp,
            OpCode::Acc => OpCode::Acc,
        };

        // execute
        if let Ok(acc) = execute_program(&program) {
            return acc;
        }
    }

    panic!("Should not reach this");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 5);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 1801);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 8);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 2060);
    }
}
