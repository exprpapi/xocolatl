use super::instructions::INSTRUCTIONS;
use super::decode::decode_instruction;

pub fn disassemble(instruction: u32) -> String {
  INSTRUCTIONS[decode_instruction(instruction) as usize].disassemble(instruction, None)
}