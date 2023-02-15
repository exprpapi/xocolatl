use super::machine::*;
use super::slices::*;

pub type Executor = &'static dyn Fn(&mut Machine, u32);

fn op_eq(a: u32, b: u32) -> u32 { (a == b) as u32 }
fn op_ne(a: u32, b: u32) -> u32 { !op_eq(a, b) }
fn op_lt(a: u32, b: u32) -> u32 { ((a as i32) < (b as i32)) as u32 }
fn op_ltu(a: u32, b: u32) -> u32 { (a < b) as u32 }
fn op_ge(a: u32, b: u32) -> u32 { !op_lt(b, a) }
fn op_geu(a: u32, b: u32) -> u32 { !op_ltu(b, a) }
fn op_add(a: u32, b: u32) -> u32 { a + b }
fn op_sub(a: u32, b: u32) -> u32 { a - b }
fn op_xor(a: u32, b: u32) -> u32 { a ^ b }
fn op_or(a: u32, b: u32) -> u32 { a | b }
fn op_and(a: u32, b: u32) -> u32 { a & b }
fn op_sll(a: u32, b: u32) -> u32 { a << (b & 0x1f) }
fn op_srl(a: u32, b: u32) -> u32 { a >> (b & 0x1f) }
fn op_sra(a: u32, b: u32) -> u32 { ((a as i32) >> (b & 0x1f)) as u32 }

const fn R_TYPE_ARITH_EXECUTOR(op: fn(u32, u32) -> u32) -> impl Fn(&mut Machine, u32) {
  move |machine, instruction| {
    let rd = get_rd_bits(instruction) as usize;
    let rs1 = machine.registers.get(get_rs1_bits(instruction) as usize);
    let rs2 = machine.registers.get(get_rs2_bits(instruction) as usize);
    machine.registers.set(rd, op(rs1, rs2));
    machine.pc += 4;
  }
}

const fn I_TYPE_ARITH_EXECUTOR(op: fn(u32, u32) -> u32) -> impl Fn(&mut Machine, u32) {
  move |machine, instruction| {
    let rd = get_rd_bits(instruction) as usize;
    let rs1 = get_rs1_bits(instruction) as usize;
    let rs1 = machine.registers.get(rs1);
    let imm = get_I_imm(instruction);
    machine.registers.set(rd, op(rs1, imm));
    machine.pc += 4;
  }
}

const fn B_TYPE_EXECUTOR(comparison: fn(u32, u32) -> u32) -> impl Fn(&mut Machine, u32) {
  move |machine, instruction| {
    let rs1 = machine.registers.get(get_rs1_bits(instruction) as usize);
    let rs2 = machine.registers.get(get_rs2_bits(instruction) as usize);
    machine.pc += if comparison(rs1, rs2) != 0 { get_imm(instruction) } else { 4 };
  }
}

pub fn LUI_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let rd = get_rd_bits(instruction) as usize;
  let imm = get_U_imm_bits(instruction);
  machine.registers.set(rd, imm);
  machine.pc += 4;
}

pub fn AUIPC_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let rd = get_rd_bits(instruction) as usize;
  let imm = get_U_imm_bits(instruction);
  machine.registers.set(rd, machine.pc + imm);
  machine.pc += 4;
}

pub fn JAL_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let rd = get_rd_bits(instruction) as usize;
  let imm = get_J_imm_bits(instruction);
  machine.registers.set(rd, machine.pc + 4);
  machine.pc += imm;
}

pub fn JALR_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let rd = get_rd_bits(instruction) as usize;
  let rs1 = machine.registers.get(get_rs1_bits(instruction) as usize);
  let imm = get_I_imm_bits(instruction);
  machine.registers.set(rd, machine.pc + 4);
  machine.pc += rs1 + imm;
}

pub const BEQ_EXECUTOR: Executor = &B_TYPE_EXECUTOR(op_eq);
pub const BNE_EXECUTOR: Executor = &B_TYPE_EXECUTOR(op_ne);
pub const BLT_EXECUTOR: Executor = &B_TYPE_EXECUTOR(op_lt);
pub const BGE_EXECUTOR: Executor = &B_TYPE_EXECUTOR(op_ge);
pub const BLTU_EXECUTOR: Executor = &B_TYPE_EXECUTOR(op_ltu);
pub const BGEU_EXECUTOR: Executor = &B_TYPE_EXECUTOR(op_geu);

pub fn LB_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let rd = get_rd_bits(instruction) as usize;
  let address = (get_rs1_bits(instruction) + get_I_imm_bits(instruction)) as usize;
  let value = machine.memory.load_byte(address) as i8 as u32;
  machine.registers.set(rd, value);
  machine.pc += 4;
}

pub fn LH_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let rd = get_rd_bits(instruction) as usize;
  let address = (get_rs1_bits(instruction) + get_I_imm_bits(instruction)) as usize;
  let value = machine.memory.load_halfword(address) as i16 as u32;
  machine.registers.set(rd, value);
  machine.pc += 4;
}

pub fn LW_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let rd = get_rd_bits(instruction) as usize;
  let address = (get_rs1_bits(instruction) + get_I_imm_bits(instruction)) as usize;
  let value = machine.memory.load_word(address);
  machine.registers.set(rd, value);
  machine.pc += 4;
}

pub fn LBU_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let rd = get_rd_bits(instruction) as usize;
  let address = (get_rs1_bits(instruction) + get_I_imm_bits(instruction)) as usize;
  let value = machine.memory.load_byte(address) as u32;
  machine.registers.set(rd, value);
  machine.pc += 4;
}

pub fn LHU_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let rd = get_rd_bits(instruction) as usize;
  let address = (get_rs1_bits(instruction) + get_I_imm_bits(instruction)) as usize;
  let value = machine.memory.load_halfword(address) as u32;
  machine.registers.set(rd, value);
  machine.pc += 4;
}

pub fn SB_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let address = (get_rs1_bits(instruction) + get_S_imm_bits(instruction)) as usize;
  let rs2 = machine.registers.get(get_rs2_bits(instruction) as usize);
  machine.memory.store_byte(address, (rs2 & 0xff) as u8);
  machine.pc += 4;
}

pub fn SH_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let address = (get_rs1_bits(instruction) + get_S_imm_bits(instruction)) as usize;
  let rs2 = machine.registers.get(get_rs2_bits(instruction) as usize);
  machine.memory.store_halfword(address, (rs2 & 0xffff) as u16);
  machine.pc += 4;
}

pub fn SW_EXECUTOR(machine: &mut Machine, instruction: u32) {
  let address = (get_rs1_bits(instruction) + get_S_imm_bits(instruction)) as usize;
  let rs2 = machine.registers.get(get_rs2_bits(instruction) as usize);
  machine.memory.store_word(address, rs2);
  machine.pc += 4;
}

pub const ADDI_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const SLTI_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_lt);
pub const SLTIU_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_ltu);
pub const XORI_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_xor);
pub const ORI_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_or);
pub const ANDI_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_and);
pub const SLLI_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_sll);
pub const SRLI_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_srl);
pub const SRAI_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_sra);
pub const ADD_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_add);
pub const SUB_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_sub);
pub const SLL_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_sll);
pub const SLT_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_lt);
pub const SLTU_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_ltu);
pub const XOR_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_xor);
pub const SRL_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_srl);
pub const SRA_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_sra);
pub const OR_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_or);
pub const AND_EXECUTOR: Executor = &R_TYPE_ARITH_EXECUTOR(op_and);

// TODO
pub const FENCE_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const FENCE_TSO_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const PAUSE_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const ECALL_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const EBREAK_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const MUL_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const MULH_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const MULHSU_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const MULHU_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const DIV_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const DIVU_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const REM_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);
pub const REMU_EXECUTOR: Executor = &I_TYPE_ARITH_EXECUTOR(op_add);

pub fn UNDEF_EXECUTOR(_machine: &mut Machine, instruction: u32) {
  panic!("UNDEF ({instruction})");
}