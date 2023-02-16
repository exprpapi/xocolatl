use super::instructions::INSTRUCTIONS;
use super::decode::decode_instruction;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn disassemble(instruction: u32) -> String {
  INSTRUCTIONS[decode_instruction(instruction) as usize].disassemble(instruction, None)
}
