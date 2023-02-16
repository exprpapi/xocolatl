use super::slices::*;

pub type Assembler = &'static dyn Fn(&[u32]) -> u32;

const fn R_TYPE_ASSEMBLER(funct3: u32, funct7: u32) -> impl Fn(&[u32]) -> u32 {
  move |operands| {
    if let [rd, rs1, rs2, ..] = *operands {
      let mut instruction = 0;
      instruction = set_opcode_bits(instruction, OPCODE_OP);
      instruction = set_funct3_bits(instruction, funct3);
      instruction = set_funct7_bits(instruction, funct7);
      instruction = set_rd_bits(instruction, rd);
      instruction = set_rs1_bits(instruction, rs1);
      instruction = set_rs2_bits(instruction, rs2);
      instruction
    } else {
      panic!("Invalid operands to R_type instruction {operands:?}");
    }
  }
}

const fn I_TYPE_ASSEMBLER(opcode: u32, funct3: u32) -> impl Fn(&[u32]) -> u32 {
  move |operands| {
    if let [rd, rs1, imm, ..] = *operands {
      let mut instruction = 0;
      instruction = set_opcode_bits(instruction, opcode);
      instruction = set_funct3_bits(instruction, funct3);
      instruction = set_rd_bits(instruction, rd);
      instruction = set_rs1_bits(instruction, rs1);
      instruction = set_I_imm_bits(instruction, imm);
      instruction
    } else {
      panic!("Invalid operands to I_type instruction {operands:?}");
    }
  }
}

const fn S_TYPE_ASSEMBLER(funct3: u32) -> impl Fn(&[u32]) -> u32 {
  move |operands: &[u32]| {
    // the order of rs2, rs1 is swapped in store instructions
    if let [rs2, rs1, imm, ..] = *operands {
      let mut instruction = 0;
      instruction = set_opcode_bits(instruction, OPCODE_STORE);
      instruction = set_funct3_bits(instruction, funct3);
      instruction = set_rs1_bits(instruction, rs1);
      instruction = set_rs2_bits(instruction, rs2);
      instruction = set_S_imm_bits(instruction, imm);
      instruction
    } else {
      panic!("Invalid operands to S_type instruction: {operands:?}");
    }
  }
}

const fn B_TYPE_ASSEMBLER(funct3: u32) -> impl Fn(&[u32]) -> u32 {
  move |operands| {
    if let [rs1, rs2, imm, ..] = *operands {
      let mut instruction = 0;
      instruction = set_opcode_bits(instruction, OPCODE_BRANCH);
      instruction = set_funct3_bits(instruction, funct3);
      instruction = set_rs1_bits(instruction, rs1);
      instruction = set_rs2_bits(instruction, rs2);
      instruction = set_B_imm_bits(instruction, imm);
      instruction
    } else {
      panic!("Invalid operands to B_type instruction: {operands:?}");
    }
  }
}

const fn U_TYPE_ASSEMBLER(opcode: u32) -> impl Fn(&[u32]) -> u32 {
  move |operands| {
    if let [rd, imm, ..] = *operands {
      let mut instruction = 0;
      instruction = set_opcode_bits(instruction, opcode);
      instruction = set_rd_bits(instruction, rd);
      instruction = set_U_imm_bits(instruction, imm);
      instruction
    } else {
      panic!("Invalid operands to U_type instruction: {operands:?}");
    }
  }
}

const fn SHIFT_IMM_ASSEMBLER(opcode: u32, funct3: u32, funct7: u32) -> impl Fn(&[u32]) -> u32 {
  move |operands| {
    if let [rd, rs1, imm, ..] = *operands {
      let mut instruction = 0;
      instruction = set_opcode_bits(instruction, opcode);
      instruction = set_funct3_bits(instruction, funct3);
      instruction = set_rd_bits(instruction, rd);
      instruction = set_rs1_bits(instruction, rs1);
      instruction = set_I_imm_bits(instruction, imm);
      // set the funct7 after the immediate to overwrite the upper bits of immediate
      instruction = set_funct7_bits(instruction, funct7);
      instruction
    } else {
      panic!("Invalid operands to shift instruction {operands:?}");
    }
  }
}

pub const LUI_ASSEMBLER: Assembler = &U_TYPE_ASSEMBLER(OPCODE_LUI);

pub const AUIPC_ASSEMBLER: Assembler = &U_TYPE_ASSEMBLER(OPCODE_AUIPC);

pub fn JAL_ASSEMBLER(operands: &[u32]) -> u32 {
  if let [rd, imm, ..] = *operands {
    let mut instruction = 0;
    instruction = set_opcode_bits(instruction, OPCODE_JAL);
    instruction = set_rd_bits(instruction, rd);
    instruction = set_J_imm_bits(instruction, imm);
    instruction
  } else {
    panic!("Invalid operands to JAL instruction: {operands:?}");
  }
}

pub const JALR_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_JALR, FUNCT3_JALR);
pub const BEQ_ASSEMBLER: Assembler = &B_TYPE_ASSEMBLER(FUNCT3_BEQ);
pub const BNE_ASSEMBLER: Assembler = &B_TYPE_ASSEMBLER(FUNCT3_BNE);
pub const BLT_ASSEMBLER: Assembler = &B_TYPE_ASSEMBLER(FUNCT3_BLT);
pub const BGE_ASSEMBLER: Assembler = &B_TYPE_ASSEMBLER(FUNCT3_BGE);
pub const BLTU_ASSEMBLER: Assembler = &B_TYPE_ASSEMBLER(FUNCT3_BLTU);
pub const BGEU_ASSEMBLER: Assembler = &B_TYPE_ASSEMBLER(FUNCT3_BGEU);
pub const LB_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_LOAD, FUNCT3_LB);
pub const LH_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_LOAD, FUNCT3_LH);
pub const LW_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_LOAD, FUNCT3_LW);
pub const LBU_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_LOAD, FUNCT3_LBU);
pub const LHU_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_LOAD, FUNCT3_LHU);
pub const SB_ASSEMBLER: Assembler = &S_TYPE_ASSEMBLER(FUNCT3_SB);
pub const SH_ASSEMBLER: Assembler = &S_TYPE_ASSEMBLER(FUNCT3_SH);
pub const SW_ASSEMBLER: Assembler = &S_TYPE_ASSEMBLER(FUNCT3_SW);
pub const ADDI_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_OP_IMM, FUNCT3_ADDI);
pub const SLTI_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_OP_IMM, FUNCT3_SLTI);
pub const SLTIU_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_OP_IMM, FUNCT3_SLTIU);
pub const XORI_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_OP_IMM, FUNCT3_XORI);
pub const ORI_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_OP_IMM, FUNCT3_ORI);
pub const ANDI_ASSEMBLER: Assembler = &I_TYPE_ASSEMBLER(OPCODE_OP_IMM, FUNCT3_ANDI);
pub const SLLI_ASSEMBLER: Assembler = &SHIFT_IMM_ASSEMBLER(OPCODE_OP_IMM, FUNCT3_SLLI, FUNCT7_SLLI);
pub const SRLI_ASSEMBLER: Assembler = &SHIFT_IMM_ASSEMBLER(OPCODE_OP_IMM, FUNCT3_SRLI_SRAI, FUNCT7_SRLI);
pub const SRAI_ASSEMBLER: Assembler = &SHIFT_IMM_ASSEMBLER(OPCODE_OP_IMM, FUNCT3_SRLI_SRAI, FUNCT7_SRAI);
pub const ADD_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_ADD, FUNCT7_ADD);
pub const SUB_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_SUB, FUNCT7_SUB);
pub const SLL_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_SLL, FUNCT7_SLL);
pub const SLT_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_SLT, FUNCT7_SLT);
pub const SLTU_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_SLTU, FUNCT7_SLTU);
pub const XOR_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_XOR, FUNCT7_XOR);
pub const SRL_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_SRL, FUNCT7_SRL);
pub const SRA_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_SRA, FUNCT7_SRA);
pub const OR_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_OR, FUNCT7_OR); 
pub const AND_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_AND, FUNCT7_AND);

pub fn FENCE_ASSEMBLER(operands: &[u32]) -> u32 {
  if let [rd, rs1, fm, pred, succ, ..] = *operands {
    let funct12 = (fm << 8) | (pred << 4) | succ;
    let mut instruction = 0;
    instruction = set_opcode_bits(instruction, OPCODE_MISC_MEM);
    instruction = set_rd_bits(instruction, rd);
    instruction = set_rs1_bits(instruction, rs1);
    instruction = set_funct12_bits(instruction, funct12);
    instruction
  } else {
    panic!("Invalid operands to FENCE instruction: {operands:?}");
  }
}

pub fn FENCE_TSO_ASSEMBLER(_operands: &[u32]) -> u32 {
  INSTRUCTION_FENCE_TSO_VALUE
}

pub fn PAUSE_ASSEMBLER(_operands: &[u32]) -> u32 {
  INSTRUCTION_PAUSE_VALUE
}

pub fn ECALL_ASSEMBLER(_operands: &[u32]) -> u32 {
  INSTRUCTION_ECALL_VALUE
}

pub fn EBREAK_ASSEMBLER(_operands: &[u32]) -> u32 {
  INSTRUCTION_EBREAK_VALUE
}

pub const MUL_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_MUL, FUNCT7_MUL);
pub const MULH_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_MULH, FUNCT7_MULH);
pub const MULHSU_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_MULHSU, FUNCT7_MULHSU);
pub const MULHU_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_MULHU, FUNCT7_MULHU);
pub const DIV_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_DIV, FUNCT7_DIV);
pub const DIVU_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_DIVU, FUNCT7_DIVU);
pub const REM_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_REM, FUNCT7_REM);
pub const REMU_ASSEMBLER: Assembler = &R_TYPE_ASSEMBLER(FUNCT3_REMU, FUNCT7_REMU);


pub fn UNDEF_ASSEMBLER(_operands: &[u32]) -> u32 {
  panic!("Invalid undef")
}
