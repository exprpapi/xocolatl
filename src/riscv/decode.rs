use super::slices::*;
use super::instructions::*;

pub fn decode(instruction: u32) -> &'static Instruction {
  let instruction_index = decode_instruction(instruction) as usize;
  &INSTRUCTIONS[instruction_index]
}

/// returns the index in the instruction table for now
pub fn decode_instruction(instruction: u32) -> InstructionIndex {
  match get_opcode_bits(instruction) {
    OPCODE_LUI => InstructionIndex::LUI,
    OPCODE_AUIPC => InstructionIndex::AUIPC,
    OPCODE_JAL => InstructionIndex::JAL,
    OPCODE_JALR => decode_opcode_jalr(instruction),
    OPCODE_BRANCH => decode_opcode_branch(instruction),
    OPCODE_LOAD => decode_opcode_load(instruction),
    OPCODE_STORE => decode_opcode_store(instruction),
    OPCODE_OP_IMM => decode_opcode_op_imm(instruction),
    OPCODE_OP => decode_opcode_op(instruction),
    OPCODE_SYSTEM => decode_opcode_system(instruction),
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_opcode_jalr(instruction: u32) -> InstructionIndex {
  if let FUNCT3_JALR = get_funct3_bits(instruction) {
    InstructionIndex::JALR
  } else {
    InstructionIndex::UNDEF
  }
}

fn decode_opcode_branch(instruction: u32) -> InstructionIndex {
  match get_funct3_bits(instruction) {
    FUNCT3_BEQ => InstructionIndex::BEQ,
    FUNCT3_BNE => InstructionIndex::BNE,
    FUNCT3_BLT => InstructionIndex::BLT,
    FUNCT3_BGE => InstructionIndex::BGE,
    FUNCT3_BLTU => InstructionIndex::BLTU,
    FUNCT3_BGEU => InstructionIndex::BGEU,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_opcode_load(instruction: u32) -> InstructionIndex {
  match get_funct3_bits(instruction) {
    FUNCT3_LB => InstructionIndex::LB,
    FUNCT3_LH => InstructionIndex::LH,
    FUNCT3_LW => InstructionIndex::LW,
    FUNCT3_LBU => InstructionIndex::LBU,
    FUNCT3_LHU => InstructionIndex::LHU,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_opcode_store(instruction: u32) -> InstructionIndex {
  match get_funct3_bits(instruction) {
    FUNCT3_SB => InstructionIndex::SB,
    FUNCT3_SH => InstructionIndex::SH,
    FUNCT3_SW => InstructionIndex::SW,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_opcode_op_imm(instruction: u32) -> InstructionIndex {
  match get_funct3_bits(instruction) {
    FUNCT3_ADDI => InstructionIndex::ADDI,
    FUNCT3_SLTI => InstructionIndex::SLTI,
    FUNCT3_SLTIU => InstructionIndex::SLTIU,
    FUNCT3_XORI => InstructionIndex::XORI,
    FUNCT3_ORI => InstructionIndex::ORI,
    FUNCT3_ANDI => InstructionIndex::ANDI,
    FUNCT3_SLLI => InstructionIndex::SLLI,
    FUNCT3_SRLI_SRAI => decode_SRLI_SRAI(instruction),
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_opcode_op(instruction: u32) -> InstructionIndex {
  match get_funct3_bits(instruction) {
    FUNCT3_ADD_SUB_MUL => decode_ADD_SUB_MUL(instruction),
    FUNCT3_SLL_MULH => decode_SLL_MULH(instruction),
    FUNCT3_SLT_MULHSU => decode_SLT_MULHSU(instruction),
    FUNCT3_SLTU_MULHU => decode_SLTU_MULHU(instruction),
    FUNCT3_XOR_DIV => decode_XOR_DIV(instruction),
    FUNCT3_SRL_SRA_DIVU => decode_SRL_SRA_DIVU(instruction),
    FUNCT3_OR_REM => decode_OR_REM(instruction),
    FUNCT3_AND_REMU => decode_AND_REMU(instruction),
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_opcode_system(instruction: u32) -> InstructionIndex {
  const EBREAK_INSTRUCTION: u32 = OPCODE_SYSTEM | (1 << 20);
  match instruction {
    OPCODE_SYSTEM => InstructionIndex::ECALL,
    EBREAK_INSTRUCTION => InstructionIndex::EBREAK,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_SRLI_SRAI(instruction: u32) -> InstructionIndex {
  match get_funct7_bits(instruction) {
    FUNCT7_SRLI => InstructionIndex::SRLI,
    FUNCT7_SRAI => InstructionIndex::SRAI,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_ADD_SUB_MUL(instruction: u32) -> InstructionIndex {
  match get_funct7_bits(instruction) {
    FUNCT7_ADD => InstructionIndex::ADD,
    FUNCT7_SUB => InstructionIndex::SUB,
    FUNCT7_MUL => InstructionIndex::MUL,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_SLL_MULH(instruction: u32) -> InstructionIndex {
  match get_funct7_bits(instruction) {
    FUNCT7_SLL => InstructionIndex::SLL,
    FUNCT7_MULH => InstructionIndex::MULH,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_SLT_MULHSU(instruction: u32) -> InstructionIndex {
  match get_funct7_bits(instruction) {
    FUNCT7_SLT => InstructionIndex::SLT,
    FUNCT7_MULHSU => InstructionIndex::MULHSU,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_SLTU_MULHU(instruction: u32) -> InstructionIndex {
  match get_funct7_bits(instruction) {
    FUNCT7_SLTU => InstructionIndex::SLTU,
    FUNCT7_MULHU => InstructionIndex::MULHU,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_XOR_DIV(instruction: u32) -> InstructionIndex {
  match get_funct7_bits(instruction) {
    FUNCT7_XOR => InstructionIndex::XOR,
    FUNCT7_DIV => InstructionIndex::DIV,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_SRL_SRA_DIVU(instruction: u32) -> InstructionIndex {
  match get_funct7_bits(instruction) {
    FUNCT7_SRL => InstructionIndex::SRL,
    FUNCT7_SRA => InstructionIndex::SRA,
    FUNCT7_DIVU => InstructionIndex::DIVU,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_OR_REM(instruction: u32) -> InstructionIndex {
  match get_funct7_bits(instruction) {
    FUNCT7_OR => InstructionIndex::OR,
    FUNCT7_REM => InstructionIndex::REM,
    _ => InstructionIndex::UNDEF,
  }
}

fn decode_AND_REMU(instruction: u32) -> InstructionIndex {
  match get_funct7_bits(instruction) {
    FUNCT7_AND => InstructionIndex::AND,
    FUNCT7_REMU => InstructionIndex::REMU,
    _ => InstructionIndex::UNDEF,
  }
}
