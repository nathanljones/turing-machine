use crate::direction::Direction;
use std::collections::HashMap;

// THis is the instruction key to be used to look up
// the correct instruction in the hashmap of instructions
pub struct InstructionKey {
    state: char,
    current_symbol: char,
}
// Next instruction to be called
pub struct Instruction {
    direction: Direction,
    write_symbol: char,
    new_state: char,
}

pub struct Instructions {
    instructions: HashMap<InstructionKey, Instruction>,
}
// get the next instruction
impl Instructions {
    pub fn get_next_instruction(
        &self,
        instruction_key: InstructionKey,
    ) -> HashMap<InstructionKey, Instruction> {
        todo!()
    }
}
