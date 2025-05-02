use crate::direction::Direction;
use std::collections::HashMap;

// THis is the instruction key to be used to look up
// the correct instruction in the hashmap of instructions
#[derive(Eq, Hash, PartialEq)]
pub struct InstructionKey {
    state: char,
    current_symbol: char,
}
// Next instruction to be called
#[derive(Copy, Clone)]
pub struct Instruction {
    direction: Direction,
    write_symbol: char,
    new_state: char,
}

pub struct Instructions {
    instructions: HashMap<InstructionKey, Instruction>,
}
impl Instructions {
    pub fn new() -> Self {
        Self {
            instructions: HashMap::new(),
        }
    }
    pub fn get_next_instruction(&self, instruction_key: InstructionKey) -> Option<Instruction> {

        match self.instructions.get(&instruction_key){
            Some(instruction) =>Some(*instruction) ,
            None =>None
        }
    }
    pub fn add_instruction(&mut self, instruction_key: InstructionKey, instruction: Instruction) {
        // add instruction to the hashmap
        self.instructions.insert(instruction_key, instruction);
    }
}
mod test {
    use super::*;
    fn setup_data()->Instructions {
        let mut instructions = Instructions::new();
        let instruction_key: InstructionKey = InstructionKey {
            state: 'A',
            current_symbol: 'B',
        };
        let instruction: Instruction = Instruction {
            direction: Direction::Right,
            write_symbol: 'C',
            new_state: 'D',
        };
        instructions.add_instruction(instruction_key, instruction);
        let instruction_key: InstructionKey = InstructionKey {
            state: 'C',
            current_symbol: 'D',
        };
        let instruction: Instruction = Instruction {
            direction: Direction::Left,
            write_symbol: 'G',
            new_state: 'H',
        };
        instructions.add_instruction(instruction_key, instruction);
        instructions
    }
    #[test]
    fn test_can_find_instruction(){
        let instructions = setup_data();
        let instruction_key: InstructionKey = InstructionKey {
            state: 'A',
            current_symbol: 'B',
        };
        let instruction = instructions.get_next_instruction(instruction_key);
        assert!(instruction.is_some());

        let instruction_key: InstructionKey = InstructionKey {
            state: 'C',
            current_symbol: 'D',
        };
        let instruction = instructions.get_next_instruction(instruction_key);
        assert!(instruction.is_some());

        let instruction_key: InstructionKey = InstructionKey {
            state: 'G',
            current_symbol: 'Q',
        };
        let instruction = instructions.get_next_instruction(instruction_key);
        assert!(!instruction.is_some());

    }

    #[test]
    fn test_is_correct_instruction_found(){
        let instructions = setup_data();
        let instruction_key: InstructionKey = InstructionKey {
            state: 'A',
            current_symbol: 'B',
        };

        // test proper begin here
        let instruction = instructions.get_next_instruction(instruction_key).unwrap();
        let direction = instruction.direction;
        let write_symbol = instruction.write_symbol;
        let new_state = instruction.new_state;
        assert_eq!(direction, Direction::Right);
        assert_eq!(write_symbol, 'C');
        assert_eq!(new_state, 'D');

        let instruction_key: InstructionKey = InstructionKey {
            state: 'C',
            current_symbol: 'D',
        };

        let instruction = instructions.get_next_instruction(instruction_key).unwrap();
        let direction = instruction.direction;
        let write_symbol = instruction.write_symbol;
        let new_state = instruction.new_state;
        assert_eq!(direction, Direction::Left);
        assert_eq!(write_symbol, 'G');
        assert_eq!(new_state, 'H');
    }

}
