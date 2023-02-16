use super::instructions::*;

/// assemble a instruction in valid normalized assembly form
/// that is, register names starting with x, no commas,
/// only separating single whitespace space characters, no labels,
/// lowercase instruction mnemonics
/// immediate offsets are not encoded like 
/// for example, `addi x1 x2 134` but not `xor a3, a5, x8` nor `add s0 t1   t2`
pub fn assemble_normalized(instruction: &str) -> u32 {
  let instruction = instruction.split(' ').collect::<Vec<_>>();
  let (instruction, operands) = (instruction[0], &instruction[1..]);
  match instruction {
    "lui" => assemble_normalized_UJ_type(INSTRUCTION_LUI, operands),
    "auipc" => assemble_normalized_UJ_type(INSTRUCTION_AUIPC, operands),
    "jal" => assemble_normalized_UJ_type(INSTRUCTION_JAL, operands),
    "jalr" => assemble_normalized_ISBJ_type(INSTRUCTION_JALR, operands),
    "beq" => assemble_normalized_ISBJ_type(INSTRUCTION_BEQ, operands),
    "bne" => assemble_normalized_ISBJ_type(INSTRUCTION_BNE, operands),
    "blt" => assemble_normalized_ISBJ_type(INSTRUCTION_BLT, operands),
    "bge" => assemble_normalized_ISBJ_type(INSTRUCTION_BGE, operands),
    "bltu" => assemble_normalized_ISBJ_type(INSTRUCTION_BLTU, operands),
    "bgeu" => assemble_normalized_ISBJ_type(INSTRUCTION_BGEU, operands),
    "lb" => assemble_normalized_ISBJ_type(INSTRUCTION_LB, operands),
    "lh" => assemble_normalized_ISBJ_type(INSTRUCTION_LH, operands),
    "lw" => assemble_normalized_ISBJ_type(INSTRUCTION_LW, operands),
    "lbu" => assemble_normalized_ISBJ_type(INSTRUCTION_LBU, operands),
    "lhu" => assemble_normalized_ISBJ_type(INSTRUCTION_LHU, operands),
    "sb" => assemble_normalized_ISBJ_type(INSTRUCTION_SB, operands),
    "sh" => assemble_normalized_ISBJ_type(INSTRUCTION_SH, operands),
    "sw" => assemble_normalized_ISBJ_type(INSTRUCTION_SW, operands),
    "addi" => assemble_normalized_ISBJ_type(INSTRUCTION_ADDI, operands),
    "slti" => assemble_normalized_ISBJ_type(INSTRUCTION_SLTI, operands),
    "sltiu" => assemble_normalized_ISBJ_type(INSTRUCTION_SLTIU, operands),
    "xori" => assemble_normalized_ISBJ_type(INSTRUCTION_XORI, operands),
    "ori" => assemble_normalized_ISBJ_type(INSTRUCTION_ORI, operands),
    "andi" => assemble_normalized_ISBJ_type(INSTRUCTION_ANDI, operands),
    "slli" => assemble_normalized_ISBJ_type(INSTRUCTION_SLLI, operands),
    "srli" => assemble_normalized_ISBJ_type(INSTRUCTION_SRLI, operands),
    "srai" => assemble_normalized_ISBJ_type(INSTRUCTION_SRAI, operands),
    "add" => assemble_normalized_R_type(INSTRUCTION_ADD, operands),
    "sub" => assemble_normalized_R_type(INSTRUCTION_SUB, operands),
    "sll" => assemble_normalized_R_type(INSTRUCTION_SLL, operands),
    "slt" => assemble_normalized_R_type(INSTRUCTION_SLT, operands),
    "sltu" => assemble_normalized_R_type(INSTRUCTION_SLTU, operands),
    "xor" => assemble_normalized_R_type(INSTRUCTION_XOR, operands),
    "srl" => assemble_normalized_R_type(INSTRUCTION_SRL, operands),
    "sra" => assemble_normalized_R_type(INSTRUCTION_SRA, operands),
    "or" => assemble_normalized_R_type(INSTRUCTION_OR, operands),
    "and" => assemble_normalized_R_type(INSTRUCTION_AND, operands),
    "fence" => assemble_normalized_FENCE(operands),
    // these take no arguments, pass in empty slice `&[]`
    "fence.tso" => (INSTRUCTION_FENCE_TSO.assembler)(&[]),
    "pause" => (INSTRUCTION_PAUSE.assembler)(&[]),
    "ecall" => (INSTRUCTION_ECALL.assembler)(&[]),
    "ebreak" => (INSTRUCTION_EBREAK.assembler)(&[]),
    "mul" => assemble_normalized_R_type(INSTRUCTION_MUL, operands),
    "mulh" => assemble_normalized_R_type(INSTRUCTION_MULH, operands),
    "mulhsu" => assemble_normalized_R_type(INSTRUCTION_MULHSU, operands),
    "mulhu" => assemble_normalized_R_type(INSTRUCTION_MULHU, operands),
    "div" => assemble_normalized_R_type(INSTRUCTION_DIV, operands),
    "divu" => assemble_normalized_R_type(INSTRUCTION_DIVU, operands),
    "rem" => assemble_normalized_R_type(INSTRUCTION_REM, operands),
    "remu" => assemble_normalized_R_type(INSTRUCTION_REMU, operands),
    _ => panic!("Invalid instruction for assembling: {instruction:?}")
  }
}

fn assemble_normalized_R_type(instruction: Instruction, operands: &[&str]) -> u32 {
  if let [rd, rs1, rs2] = *operands {
    let rd = parse_normalized_register(rd);
    let rs1 = parse_normalized_register(rs1);
    let rs2 = parse_normalized_register(rs2);
    let operands = &[rd, rs1, rs2];
    (instruction.assembler)(operands)
  } else {
    panic!("Invalid operands to instruction {0}: {operands:?}", instruction.name);
  }
}

fn assemble_normalized_ISBJ_type(instruction: Instruction, operands: &[&str]) -> u32 {
  if let [rd, rs1, imm] = *operands {
    let rd = parse_normalized_register(rd);
    let rs1 = parse_normalized_register(rs1);
    let imm = imm.parse::<i32>().unwrap() as u32;
    let operands = &[rd, rs1, imm];
    (instruction.assembler)(operands)
  } else {
    panic!("Invalid operands to instruction {0}: {operands:?}", instruction.name);
  }
}

fn assemble_normalized_UJ_type(instruction: Instruction, operands: &[&str]) -> u32 {
  if let [rd, imm] = *operands {
    let rd = parse_normalized_register(rd);
    let imm = imm.parse::<i32>().unwrap() as u32;
    let operands = &[rd, imm];
    (instruction.assembler)(operands)
  } else {
    panic!("Invalid operands to instruction {0}: {operands:?}", instruction.name);
  }
}

fn assemble_normalized_FENCE(operands: &[&str]) -> u32 {
  let instruction = INSTRUCTION_FENCE;
  if let [rd, rs1, fm, pred, succ] = operands[..5] {
    let rd = parse_normalized_register(rd);
    let rs1 = parse_normalized_register(rs1);
    let fm = fm.parse::<u32>().unwrap();
    let pred = pred.parse::<u32>().unwrap();
    let succ = succ.parse::<u32>().unwrap();
    let operands = &[rd, rs1, fm, pred, succ];
    (instruction.assembler)(operands)
  } else {
    panic!("Invalid operands to instruction {0}: {operands:?}", instruction.name);
  }
}

pub fn assemble_program(program: Vec<&str>) -> Vec<u32> {
  program.iter().map(|instruction| assemble_normalized(instruction)).collect()
}

fn parse_normalized_register(register: &str) -> u32 {
  register[1..].parse().expect("could not parse register {register}")
}

fn _parse_immediate(_imm: &str) -> u32 {
  1
}

// fn parse_program(program: &str) -> Vec<(Instruction, &[u32])> {
//   program.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).map(parse_instruction).collect()
// }

// fn parse_instruction(instruction: &str) -> Vec<(Instruction, &[u32])> {
//   vec![INSTRUCTION_ADD]
// }
