const fn slice_mask(lo: usize, hi: usize) -> u32 {
  assert!(lo <= hi && hi <= 31);
  (u32::MAX >> (31 - hi)) & (u32::MAX << lo)
}

#[test]
fn test_slice_mask() {
  assert_eq!(0xffff, slice_mask(0, 15));
  assert_eq!(0xffff_ffff, slice_mask(0, 31));
  assert_eq!(0x1f0, slice_mask(4, 8));
  assert_eq!(0b1101110010, (0x6e50206f & slice_mask(21, 31)) >> 21);
  assert_eq!(1, (12004 & slice_mask(11, 11)) >> 11);
}

pub const OPCODE_LUI: u32 = 0x37;
pub const OPCODE_AUIPC: u32 = 0x17;
pub const OPCODE_JAL: u32 = 0x6f;
pub const OPCODE_JALR: u32 = 0x67;
pub const OPCODE_BRANCH: u32 = 0x63;
pub const OPCODE_LOAD: u32 = 0x03;
pub const OPCODE_STORE: u32 = 0x23;
pub const OPCODE_OP_IMM: u32 = 0x13;
pub const OPCODE_OP: u32 = 0x33;
pub const OPCODE_MISC_MEM: u32 = 0xf;
pub const OPCODE_SYSTEM: u32 = 0x73;

pub const FUNCT3_JALR: u32 = 0;
pub const FUNCT3_BEQ: u32 = 0;
pub const FUNCT3_BNE: u32 = 1;
pub const FUNCT3_BLT: u32 = 4;
pub const FUNCT3_BGE: u32 = 5;
pub const FUNCT3_BLTU: u32 = 6;
pub const FUNCT3_BGEU: u32 = 7;
pub const FUNCT3_LB: u32 = 0;
pub const FUNCT3_LH: u32 = 1;
pub const FUNCT3_LW: u32 = 2;
pub const FUNCT3_LBU: u32 = 4;
pub const FUNCT3_LHU: u32 = 5;
pub const FUNCT3_SB: u32 = 0;
pub const FUNCT3_SH: u32 = 1;
pub const FUNCT3_SW: u32 = 2;
pub const FUNCT3_ADDI: u32 = 0;
pub const FUNCT3_SLTI: u32 = 2;
pub const FUNCT3_SLTIU: u32 = 3;
pub const FUNCT3_XORI: u32 = 4;
pub const FUNCT3_ORI: u32 = 6;
pub const FUNCT3_ANDI: u32 = 7;
pub const FUNCT3_SLLI: u32 = 1;
pub const FUNCT3_SRLI: u32 = 5;
pub const FUNCT3_SRAI: u32 = FUNCT3_SRLI;
pub const FUNCT3_ADD: u32 = 0;
pub const FUNCT3_SUB: u32 = FUNCT3_ADD;
pub const FUNCT3_SLL: u32 = 1;
pub const FUNCT3_SLT: u32 = 2;
pub const FUNCT3_SLTU: u32 = 3;
pub const FUNCT3_XOR: u32 = 4;
pub const FUNCT3_SRL: u32 = 5;
pub const FUNCT3_SRA: u32 = FUNCT3_SRL;
pub const FUNCT3_OR: u32 = 6;
pub const FUNCT3_AND: u32 = 7;
pub const FUNCT3_MUL: u32 = FUNCT3_ADD;
pub const FUNCT3_MULH: u32 = FUNCT3_SLL;
pub const FUNCT3_MULHSU: u32 = FUNCT3_SLT;
pub const FUNCT3_MULHU: u32 = FUNCT3_SLTU;
pub const FUNCT3_DIV: u32 = FUNCT3_XOR;
pub const FUNCT3_DIVU: u32 = FUNCT3_SRL;
pub const FUNCT3_REM: u32 = FUNCT3_OR;
pub const FUNCT3_REMU: u32 = FUNCT3_AND;
// redundance intended for clearer decode
pub const FUNCT3_SRLI_SRAI: u32 = FUNCT3_SRLI;
pub const FUNCT3_ADD_SUB_MUL: u32 = FUNCT3_ADD;
pub const FUNCT3_SLL_MULH: u32 = FUNCT3_SLL;
pub const FUNCT3_SLT_MULHSU: u32 = FUNCT3_SLT;
pub const FUNCT3_SLTU_MULHU: u32 = FUNCT3_SLTU;
pub const FUNCT3_XOR_DIV: u32 = FUNCT3_XOR;
pub const FUNCT3_SRL_SRA_DIVU: u32 = FUNCT3_SRL;
pub const FUNCT3_OR_REM: u32 = FUNCT3_OR;
pub const FUNCT3_AND_REMU: u32 = FUNCT3_AND;

pub const FUNCT7_SLLI: u32 = 0;
pub const FUNCT7_SRLI: u32 = 0;
pub const FUNCT7_SRAI: u32 = slice_mask(5, 5);
pub const FUNCT7_ADD: u32 = 0;
pub const FUNCT7_SUB: u32 = slice_mask(5, 5); // 0b100000;
pub const FUNCT7_SLL: u32 = 0;
pub const FUNCT7_SLT: u32 = 0;
pub const FUNCT7_SLTU: u32 = 0;
pub const FUNCT7_XOR: u32 = 0;
pub const FUNCT7_SRL: u32 = 0;
pub const FUNCT7_SRA: u32 = slice_mask(5, 5);
pub const FUNCT7_OR: u32 = 0;
pub const FUNCT7_AND: u32 = 0;
pub const FUNCT7_MUL: u32 = 1;
pub const FUNCT7_MULH: u32 = 1;
pub const FUNCT7_MULHSU: u32 = 1;
pub const FUNCT7_MULHU: u32 = 1;
pub const FUNCT7_DIV: u32 = 1;
pub const FUNCT7_DIVU: u32 = 1;
pub const FUNCT7_REM: u32 = 1;
pub const FUNCT7_REMU: u32 = 1;

pub const FUNCT12_FENCE_TSO: u32 = 0b1000_0011_0011;
pub const FUNCT12_PAUSE: u32 = 0b0000_0001_0000;
pub const FUNCT12_ECALL: u32 = 0;
pub const FUNCT12_EBREAK: u32 = 1;

const MASK_OPCODE: u32 = slice_mask(0, 6);
const MASK_RD: u32 = slice_mask(7, 11);
const MASK_RS1: u32 = slice_mask(15, 19);
const MASK_RS2: u32 = slice_mask(20, 24);
const MASK_FUNCT3: u32 = slice_mask(12, 14);
const MASK_FUNCT7: u32 = slice_mask(25, 31);
const MASK_FUNCT12: u32 = slice_mask(20, 31);
const MASK_I_IMM_PART0: u32 = slice_mask(20, 31);
const MASK_S_IMM: u32 = MASK_S_IMM_PART1 | MASK_S_IMM_PART0;
const MASK_S_IMM_PART0: u32 = slice_mask(7, 11);
const MASK_S_IMM_PART1: u32 = slice_mask(25, 31);
const MASK_B_IMM: u32 = MASK_B_IMM_PART0 | MASK_B_IMM_PART1 | MASK_B_IMM_PART2 | MASK_B_IMM_PART3;
const MASK_B_IMM_PART0: u32 = slice_mask(8, 11);
const MASK_B_IMM_PART1: u32 = slice_mask(25, 30);
const MASK_B_IMM_PART2: u32 = slice_mask(7, 7);
const MASK_B_IMM_PART3: u32 = slice_mask(31, 31);
const MASK_U_IMM: u32 = MASK_U_IMM_PART0;
const MASK_U_IMM_PART0: u32 = slice_mask(12, 31);
const MASK_J_IMM: u32 = MASK_J_IMM_PART3 | MASK_J_IMM_PART2 | MASK_J_IMM_PART1 | MASK_J_IMM_PART0;
const MASK_J_IMM_PART0: u32 = slice_mask(21, 30);
const MASK_J_IMM_PART1: u32 = slice_mask(20, 20);
const MASK_J_IMM_PART2: u32 = slice_mask(12, 19);
const MASK_J_IMM_PART3: u32 = slice_mask(31, 31);

const OFFSET_OPCODE: u32 = 0;
const OFFSET_RD: u32 = 7;
const OFFSET_RS1: u32 = 15;
const OFFSET_RS2: u32 = 20;
const OFFSET_FUNCT3: u32 = 12;
const OFFSET_FUNCT7: u32 = 25;
const OFFSET_FUNCT12: u32 = 20;
const OFFSET_I_IMM_PART0: u32 = 20;
const OFFSET_S_IMM_PART0: u32 = 7;
const OFFSET_S_IMM_PART1: u32 = 25;
const OFFSET_B_IMM_PART0: u32 = 8;
const OFFSET_B_IMM_PART1: u32 = 25;
const OFFSET_B_IMM_PART2: u32 = 7;
const OFFSET_B_IMM_PART3: u32 = 31;
const OFFSET_U_IMM_PART0: u32 = 12;
const OFFSET_J_IMM_PART0: u32 = 21;
const OFFSET_J_IMM_PART1: u32 = 20;
const OFFSET_J_IMM_PART2: u32 = 12;
const OFFSET_J_IMM_PART3: u32 = 31;

const SLICE_OPCODE: (u32, u32) = (MASK_OPCODE, OFFSET_OPCODE);
const SLICE_RD: (u32, u32) = (MASK_RD, OFFSET_RD);
const SLICE_RS1: (u32, u32) = (MASK_RS1, OFFSET_RS1);
const SLICE_RS2: (u32, u32) = (MASK_RS2, OFFSET_RS2);
const SLICE_FUNCT3: (u32, u32) = (MASK_FUNCT3, OFFSET_FUNCT3);
const SLICE_FUNCT7: (u32, u32) = (MASK_FUNCT7, OFFSET_FUNCT7);
const SLICE_FUNCT12: (u32, u32) = (MASK_FUNCT12, OFFSET_FUNCT12);
const SLICE_I_IMM: (u32, u32) = SLICE_I_IMM_PART0;
const SLICE_I_IMM_PART0: (u32, u32) = (MASK_I_IMM_PART0, OFFSET_I_IMM_PART0);
const SLICE_S_IMM_PART0: (u32, u32) = (MASK_S_IMM_PART0, OFFSET_S_IMM_PART0);
const SLICE_S_IMM_PART1: (u32, u32) = (MASK_S_IMM_PART1, OFFSET_S_IMM_PART1);
const SLICE_B_IMM_PART0: (u32, u32) = (MASK_B_IMM_PART0, OFFSET_B_IMM_PART0);
const SLICE_B_IMM_PART1: (u32, u32) = (MASK_B_IMM_PART1, OFFSET_B_IMM_PART1);
const SLICE_B_IMM_PART2: (u32, u32) = (MASK_B_IMM_PART2, OFFSET_B_IMM_PART2);
const SLICE_B_IMM_PART3: (u32, u32) = (MASK_B_IMM_PART3, OFFSET_B_IMM_PART3);
const SLICE_U_IMM_PART0: (u32, u32) = (MASK_U_IMM_PART0, OFFSET_U_IMM_PART0);
const SLICE_J_IMM_PART0: (u32, u32) = (MASK_J_IMM_PART0, OFFSET_J_IMM_PART0);
const SLICE_J_IMM_PART1: (u32, u32) = (MASK_J_IMM_PART1, OFFSET_J_IMM_PART1);
const SLICE_J_IMM_PART2: (u32, u32) = (MASK_J_IMM_PART2, OFFSET_J_IMM_PART2);
const SLICE_J_IMM_PART3: (u32, u32) = (MASK_J_IMM_PART3, OFFSET_J_IMM_PART3);

const POSITION_I_IMM_PART0: u32 = 0;
const POSITION_S_IMM_PART0: u32 = 0;
const POSITION_S_IMM_PART1: u32 = 5;
const POSITION_B_IMM_PART0: u32 = 1;
const POSITION_B_IMM_PART1: u32 = 5;
const POSITION_B_IMM_PART2: u32 = 11;
const POSITION_B_IMM_PART3: u32 = 12;
const POSITION_U_IMM_PART0: u32 = 12;
const POSITION_J_IMM_PART0: u32 = 1;
const POSITION_J_IMM_PART1: u32 = 11;
const POSITION_J_IMM_PART2: u32 = 12;
const POSITION_J_IMM_PART3: u32 = 20;

// slices for setting non-contiguous bit slice combinations (S, B, U, J)
const MASK_S_IMM_PART0_SET: u32 = slice_mask(0, 4);
const MASK_S_IMM_PART1_SET: u32 = slice_mask(5, 11);
const MASK_B_IMM_PART0_SET: u32 = slice_mask(11, 11);
const MASK_B_IMM_PART1_SET: u32 = slice_mask(1, 4);
const MASK_B_IMM_PART2_SET: u32 = slice_mask(5, 10);
const MASK_B_IMM_PART3_SET: u32 = slice_mask(12, 12);
const MASK_J_IMM_PART0_SET: u32 = slice_mask(12, 19);
const MASK_J_IMM_PART1_SET: u32 = slice_mask(11, 11);
const MASK_J_IMM_PART2_SET: u32 = slice_mask(1, 10);
const MASK_J_IMM_PART3_SET: u32 = slice_mask(20, 20);
const OFFSET_S_IMM_PART0_SET: u32 = 0;
const OFFSET_S_IMM_PART1_SET: u32 = 5;
const OFFSET_B_IMM_PART0_SET: u32 = 11;
const OFFSET_B_IMM_PART1_SET: u32 = 1;
const OFFSET_B_IMM_PART2_SET: u32 = 5;
const OFFSET_B_IMM_PART3_SET: u32 = 12;
const OFFSET_J_IMM_PART0_SET: u32 = 12;
const OFFSET_J_IMM_PART1_SET: u32 = 11;
const OFFSET_J_IMM_PART2_SET: u32 = 1;
const OFFSET_J_IMM_PART3_SET: u32 = 20;
const POSITION_S_IMM_PART0_SET: u32 = 7;
const POSITION_S_IMM_PART1_SET: u32 = 25;
const POSITION_B_IMM_PART0_SET: u32 = 7;
const POSITION_B_IMM_PART1_SET: u32 = 8;
const POSITION_B_IMM_PART2_SET: u32 = 25;
const POSITION_B_IMM_PART3_SET: u32 = 31;
const POSITION_U_IMM_PART0_SET: u32 = 12;
const POSITION_J_IMM_PART0_SET: u32 = 12;
const POSITION_J_IMM_PART1_SET: u32 = 20;
const POSITION_J_IMM_PART2_SET: u32 = 21;
const POSITION_J_IMM_PART3_SET: u32 = 31;
const SLICE_S_IMM_PART0_SET: (u32, u32) = (MASK_S_IMM_PART0_SET, OFFSET_S_IMM_PART0_SET);
const SLICE_S_IMM_PART1_SET: (u32, u32) = (MASK_S_IMM_PART1_SET, OFFSET_S_IMM_PART1_SET);
const SLICE_B_IMM_PART0_SET: (u32, u32) = (MASK_B_IMM_PART0_SET, OFFSET_B_IMM_PART0_SET);
const SLICE_B_IMM_PART1_SET: (u32, u32) = (MASK_B_IMM_PART1_SET, OFFSET_B_IMM_PART1_SET);
const SLICE_B_IMM_PART2_SET: (u32, u32) = (MASK_B_IMM_PART2_SET, OFFSET_B_IMM_PART2_SET);
const SLICE_B_IMM_PART3_SET: (u32, u32) = (MASK_B_IMM_PART3_SET, OFFSET_B_IMM_PART3_SET);
const SLICE_J_IMM_PART0_SET: (u32, u32) = (MASK_J_IMM_PART0_SET, OFFSET_J_IMM_PART0_SET);
const SLICE_J_IMM_PART1_SET: (u32, u32) = (MASK_J_IMM_PART1_SET, OFFSET_J_IMM_PART1_SET);
const SLICE_J_IMM_PART2_SET: (u32, u32) = (MASK_J_IMM_PART2_SET, OFFSET_J_IMM_PART2_SET);
const SLICE_J_IMM_PART3_SET: (u32, u32) = (MASK_J_IMM_PART3_SET, OFFSET_J_IMM_PART3_SET);

const SXT_BIT_I_IMM: u32 = 11;
const SXT_BIT_S_IMM: u32 = 11;
const SXT_BIT_B_IMM: u32 = 12;
const SXT_BIT_U_IMM: u32 = 31;
const SXT_BIT_J_IMM: u32 = 20;

pub const INSTRUCTION_FENCE_TSO_VALUE: u32 = (FUNCT12_FENCE_TSO << 20) | OPCODE_MISC_MEM;
pub const INSTRUCTION_PAUSE_VALUE: u32 = (FUNCT12_PAUSE << 20) | OPCODE_MISC_MEM;
pub const INSTRUCTION_ECALL_VALUE: u32 = (FUNCT12_ECALL << 20) | OPCODE_SYSTEM;
pub const INSTRUCTION_EBREAK_VALUE: u32 = (FUNCT12_EBREAK << 20) | OPCODE_SYSTEM;

/// copies the bit of `x` at position `pos` into higher bits
pub fn sign_extend(x: &mut u32, pos: u32) {
  let shamt = 31 - pos;
  *x = ((*x << shamt) as i32 >> shamt) as u32
}

fn get_slice_bits(instruction: u32, slice: (u32, u32)) -> u32 {
  let (mask, offset) = slice;
  (instruction & mask) >> offset
}

/// set bits selected by mask to the corresponding bits of value,
/// which is shifted into position
fn set_slice_bits(instruction: u32, value: u32, slice: (u32, u32)) -> u32 {
  let (mask, offset) = slice;
  (instruction & !mask) | ((value << offset) & mask)
}

#[test]
fn test_set_slice_bits() {
  let x = 0xffacff;
  let mut y = 0xff00ff;
  let mask = 0xff00;
  let offset = 8;
  let value = 0xac;
  let slice = (mask, offset);
  y = set_slice_bits(y, value, slice);
  assert_eq!(x, y, "{x:#010x} != {y:#010x}");
}

pub fn get_opcode_bits(instruction: u32) -> u32 {
  get_slice_bits(instruction, SLICE_OPCODE)
}

pub fn get_funct3_bits(instruction: u32) -> u32 {
  get_slice_bits(instruction, SLICE_FUNCT3)
}

pub fn get_funct7_bits(instruction: u32) -> u32 {
  get_slice_bits(instruction, SLICE_FUNCT7)
}

pub fn _get_funct12_bits(instruction: u32) -> u32 {
  get_slice_bits(instruction, SLICE_FUNCT12)
}

pub fn get_rd_bits(instruction: u32) -> u32 {
  get_slice_bits(instruction, SLICE_RD)
}

pub fn get_rs1_bits(instruction: u32) -> u32 {
  get_slice_bits(instruction, SLICE_RS1)
}

pub fn get_rs2_bits(instruction: u32) -> u32 {
  get_slice_bits(instruction, SLICE_RS2)
}

pub fn get_I_imm_bits(instruction: u32) -> u32 {
  get_slice_bits(instruction, SLICE_I_IMM_PART0) << POSITION_I_IMM_PART0
}

pub fn get_S_imm_bits(instruction: u32) -> u32 {
  let part0 = get_slice_bits(instruction, SLICE_S_IMM_PART0) << POSITION_S_IMM_PART0;
  let part1 = get_slice_bits(instruction, SLICE_S_IMM_PART1) << POSITION_S_IMM_PART1;
  part1 | part0
}

pub fn get_B_imm_bits(instruction: u32) -> u32 {
  let part0 = get_slice_bits(instruction, SLICE_B_IMM_PART0) << POSITION_B_IMM_PART0;
  let part1 = get_slice_bits(instruction, SLICE_B_IMM_PART1) << POSITION_B_IMM_PART1;
  let part2 = get_slice_bits(instruction, SLICE_B_IMM_PART2) << POSITION_B_IMM_PART2;
  let part3 = get_slice_bits(instruction, SLICE_B_IMM_PART3) << POSITION_B_IMM_PART3;
  part3 | part2 | part1 | part0
}

pub fn get_U_imm_bits(instruction: u32) -> u32 {
  get_slice_bits(instruction, SLICE_U_IMM_PART0) << POSITION_U_IMM_PART0
}

pub fn get_J_imm_bits(instruction: u32) -> u32 {
  let part0 = get_slice_bits(instruction, SLICE_J_IMM_PART0) << POSITION_J_IMM_PART0;
  let part1 = get_slice_bits(instruction, SLICE_J_IMM_PART1) << POSITION_J_IMM_PART1;
  let part2 = get_slice_bits(instruction, SLICE_J_IMM_PART2) << POSITION_J_IMM_PART2;
  let part3 = get_slice_bits(instruction, SLICE_J_IMM_PART3) << POSITION_J_IMM_PART3;
  part3 | part2 | part1 | part0
}

pub fn get_I_imm(instruction: u32) -> u32 {
  let mut imm_bits = get_I_imm_bits(instruction);
  sign_extend(&mut imm_bits, SXT_BIT_I_IMM);
  imm_bits
}

pub fn get_S_imm(instruction: u32) -> u32 {
  let mut imm_bits = get_S_imm_bits(instruction);
  sign_extend(&mut imm_bits, SXT_BIT_S_IMM);
  imm_bits
}

pub fn get_U_imm(instruction: u32) -> u32 {
  let mut imm_bits = get_U_imm_bits(instruction);
  sign_extend(&mut imm_bits, SXT_BIT_U_IMM);
  imm_bits
}

pub fn get_B_imm(instruction: u32) -> u32 {
  let mut imm_bits = get_B_imm_bits(instruction);
  sign_extend(&mut imm_bits, SXT_BIT_B_IMM);
  imm_bits
}

pub fn get_J_imm(instruction: u32) -> u32 {
  let mut imm_bits = get_J_imm_bits(instruction);
  sign_extend(&mut imm_bits, SXT_BIT_J_IMM);
  imm_bits
}

#[test]
fn test_get_J_imm() {
  assert_eq!(12004, get_J_imm(0x6e50206f)); // jal x0 12004
  assert_eq!(184, get_J_imm(0x0b8008ef)); // jal x17 184
  assert_eq!(-69112, get_J_imm(0xa08efc6f) as i32); // jal x24 -69112
}

pub fn set_opcode_bits(instruction: u32, value: u32) -> u32 {
  set_slice_bits(instruction, value, SLICE_OPCODE)
}

pub fn set_funct3_bits(instruction: u32, value: u32) -> u32 {
  set_slice_bits(instruction, value, SLICE_FUNCT3)
}

pub fn set_funct7_bits(instruction: u32, value: u32) -> u32 {
  set_slice_bits(instruction, value, SLICE_FUNCT7)
}

pub fn set_funct12_bits(instruction: u32, value: u32) -> u32 {
  set_slice_bits(instruction, value, SLICE_FUNCT12)
}

pub fn set_rd_bits(instruction: u32, value: u32) -> u32 {
  set_slice_bits(instruction, value, SLICE_RD)
}

pub fn set_rs1_bits(instruction: u32, value: u32) -> u32 {
  set_slice_bits(instruction, value, SLICE_RS1)
}

pub fn set_rs2_bits(instruction: u32, value: u32) -> u32 {
  set_slice_bits(instruction, value, SLICE_RS2)
}

pub fn set_I_imm_bits(instruction: u32, value: u32) -> u32 {
  set_slice_bits(instruction, value, SLICE_I_IMM)
}

pub fn set_S_imm_bits(instruction: u32, value: u32) -> u32 {
  let instruction = instruction & !MASK_S_IMM;
  let part0 = get_slice_bits(value, SLICE_S_IMM_PART0_SET) << POSITION_S_IMM_PART0_SET; 
  let part1 = get_slice_bits(value, SLICE_S_IMM_PART1_SET) << POSITION_S_IMM_PART1_SET; 
  instruction | part1 | part0
}

pub fn set_B_imm_bits(instruction: u32, value: u32) -> u32 {
  let instruction = instruction & !MASK_B_IMM;
  let part0 = get_slice_bits(value, SLICE_B_IMM_PART0_SET) << POSITION_B_IMM_PART0_SET;
  let part1 = get_slice_bits(value, SLICE_B_IMM_PART1_SET) << POSITION_B_IMM_PART1_SET;
  let part2 = get_slice_bits(value, SLICE_B_IMM_PART2_SET) << POSITION_B_IMM_PART2_SET;
  let part3 = get_slice_bits(value, SLICE_B_IMM_PART3_SET) << POSITION_B_IMM_PART3_SET;
  instruction | part3 | part2 | part1 | part0
}

pub fn set_U_imm_bits(instruction: u32, value: u32) -> u32 {
  let instruction = instruction & !MASK_U_IMM;
  let part0 =  value << POSITION_U_IMM_PART0_SET;
  instruction | part0
}

pub fn set_J_imm_bits(instruction: u32, value: u32) -> u32 {
  let instruction = instruction & !MASK_J_IMM;
  let part0 = get_slice_bits(value, SLICE_J_IMM_PART0_SET) << POSITION_J_IMM_PART0_SET;
  let part1 = get_slice_bits(value, SLICE_J_IMM_PART1_SET) << POSITION_J_IMM_PART1_SET;
  let part2 = get_slice_bits(value, SLICE_J_IMM_PART2_SET) << POSITION_J_IMM_PART2_SET;
  let part3 = get_slice_bits(value, SLICE_J_IMM_PART3_SET) << POSITION_J_IMM_PART3_SET;
  instruction | part3 | part2 | part1 | part0
}

#[test]
fn test_set_J_imm_bits() {
  assert_eq!(format!("{0:#034b}", 0x6e50206f), format!("{0:#034b}", set_J_imm_bits(OPCODE_JAL, 12004))); // jal x0 12004
  assert_eq!(format!("{0:#034b}", 0x0b8008ef), format!("{0:#034b}", set_J_imm_bits(OPCODE_JAL | 17 << 7, 184))); // jal x17 184
  assert_eq!(format!("{0:#034b}", 0xa08efc6f_u32), format!("{0:#034b}", set_J_imm_bits(OPCODE_JAL | 24 << 7, (-69112_i32) as u32))); // jal x24 -69112
}

pub fn get_imm(instruction: u32) -> u32 {
  let opcode = get_opcode_bits(instruction); 
  match opcode {
    OPCODE_LUI => get_U_imm(instruction),
    OPCODE_AUIPC => get_U_imm(instruction),
    OPCODE_JAL => get_J_imm(instruction),
    OPCODE_JALR => get_I_imm(instruction),
    OPCODE_BRANCH => get_B_imm(instruction),
    OPCODE_LOAD => get_I_imm(instruction),
    OPCODE_STORE => get_S_imm(instruction),
    OPCODE_OP_IMM => get_I_imm(instruction),
    _ => panic!("bad opcode {opcode} for instruction {instruction}"),
  }
}
