use super::machine::*;
use super::slices::*;

pub type Disassembler = &'static dyn Fn(u32, Option<&Machine>) -> String;

const fn R_TYPE_DISASSEMBLER(name: &str) -> impl Fn(u32, Option<&Machine>) -> String + '_ {
  move |instruction, machine| {
    let name = name.to_owned();
    let rd = get_register_name(get_rd_bits(instruction) as usize);
    let rs1_raw = get_rs1_bits(instruction) as usize;
    let rs2_raw = get_rs2_bits(instruction) as usize;
    let mut rs1 = get_register_name(rs1_raw);
    let mut rs2 = get_register_name(rs2_raw);
    if let Some(machine) = machine {
      let rs1_value = machine.registers.get(rs1_raw);
      let rs2_value = machine.registers.get(rs2_raw);
      rs1.push_str(&format!("={rs1_value}"));
      rs2.push_str(&format!("={rs2_value}"));
    };
    format!("{name} {rd}, {rs1}, {rs2}")
  }
}

const fn I_TYPE_DISASSEMBLER(name: &str) -> impl Fn(u32, Option<&Machine>) -> String + '_ {
  move |instruction, machine| {
    let name = name.to_owned();
    let rd = get_register_name(get_rd_bits(instruction) as usize);
    let rs1_raw = get_rs1_bits(instruction) as usize;
    let mut rs1 = get_register_name(rs1_raw);
    let imm = get_I_imm_bits(instruction);
    if let Some(machine) = machine {
      let rs1_value = machine.registers.get(rs1_raw);
      rs1.push_str(&format!("={rs1_value}"));
    };
    format!("{name} {rd}, {rs1}, {imm}")
  }
}

const fn S_TYPE_DISASSEMBLER(name: &str) -> impl Fn(u32, Option<&Machine>) -> String + '_ {
  move |instruction, machine| {
    let name = name.to_owned();
    let imm = get_S_imm_bits(instruction);
    let rs1_raw = get_rs1_bits(instruction) as usize;
    let rs2_raw = get_rs2_bits(instruction) as usize;
    let mut rs1 = get_register_name(rs1_raw);
    let mut rs2 = get_register_name(rs2_raw);
    if let Some(machine) = machine {
      let rs1_value = machine.registers.get(rs1_raw);
      let rs2_value = machine.registers.get(rs2_raw);
      rs1.push_str(&format!("={rs1_value}"));
      rs2.push_str(&format!("={rs2_value}"));
    };
    format!("{name} {rs2}, {imm}({rs1})")
  }
}

const fn B_TYPE_DISASSEMBLER(name: &str) -> impl Fn(u32, Option<&Machine>) -> String + '_ {
  move |instruction, machine| {
    let name = name.to_owned();
    let rs1_raw = get_rs1_bits(instruction) as usize;
    let rs2_raw = get_rs2_bits(instruction) as usize;
    let mut rs1 = get_register_name(rs1_raw);
    let mut rs2 = get_register_name(rs2_raw);
    let offset = get_B_imm_bits(instruction);
    if let Some(machine) = machine {
      let rs1_value = machine.registers.get(rs1_raw);
      let rs2_value = machine.registers.get(rs2_raw);
      rs1.push_str(&format!("={rs1_value}"));
      rs2.push_str(&format!("={rs2_value}"));
    };
    format!("{name} {rs1}, {rs2}, {offset:#0x}")
  }
}

const fn U_TYPE_DISASSEMBLER(name: &str) -> impl Fn(u32, Option<&Machine>) -> String + '_ {
  move |instruction, _machine| {
    let name = name.to_owned();
    let rd = get_register_name(get_rd_bits(instruction) as usize);
    let imm = get_U_imm_bits(instruction) >> 12;
    format!("{name} {rd}, {imm}")
  }
}

const fn J_TYPE_DISASSEMBLER(name: &str) -> impl Fn(u32, Option<&Machine>) -> String + '_ {
  move |instruction, _machine| {
    let name = name.to_owned();
    let rd = get_register_name(get_rd_bits(instruction) as usize);
    let imm = get_J_imm_bits(instruction);
    format!("{name} {rd}, {imm}")
  }
}

const fn LOAD_DISASSEMBLER(name: &str) -> impl Fn(u32, Option<&Machine>) -> String + '_ {
  move |instruction, machine| {
    let name = name.to_owned();
    let rd = get_register_name(get_rd_bits(instruction) as usize);
    let rs1_raw = get_rs1_bits(instruction) as usize;
    let mut rs1 = get_register_name(rs1_raw);
    let imm = get_I_imm_bits(instruction);
    if let Some(machine) = machine {
      let rs1_value = machine.registers.get(rs1_raw);
      rs1.push_str(&format!("={rs1_value}"));
    };
    format!("{name} {rd}, {imm}({rs1})")
  }
}

const fn SYSTEM_DISASSEMBLER(name: &str) -> impl Fn(u32, Option<&Machine>) -> String + '_ {
  move |_instruction, _machine| {
    name.to_owned()
  }
}

pub const LUI_DISASSEMBLER: Disassembler = &U_TYPE_DISASSEMBLER("lui");
pub const AUIPC_DISASSEMBLER: Disassembler = &U_TYPE_DISASSEMBLER("auipc");
pub const JAL_DISASSEMBLER: Disassembler = &J_TYPE_DISASSEMBLER("jal");

pub fn JALR_DISASSEMBLER(instruction: u32, machine: Option<&Machine>) -> String {
  let rd = get_register_name(get_rd_bits(instruction) as usize);
  let rs1_raw = get_rs1_bits(instruction) as usize;
  let mut rs1 = get_register_name(rs1_raw);
  let imm = get_I_imm_bits(instruction);
  if let Some(machine) = machine {
    let rs1_value = machine.registers.get(rs1_raw);
    rs1.push_str(&format!("={rs1_value}"));
  };
  format!("jalr {rd}, {imm}({rs1})")
}

pub const BEQ_DISASSEMBLER: Disassembler = &B_TYPE_DISASSEMBLER("beq");
pub const BNE_DISASSEMBLER: Disassembler = &B_TYPE_DISASSEMBLER("bne");
pub const BLT_DISASSEMBLER: Disassembler = &B_TYPE_DISASSEMBLER("blt");
pub const BGE_DISASSEMBLER: Disassembler = &B_TYPE_DISASSEMBLER("bge");
pub const BLTU_DISASSEMBLER: Disassembler = &B_TYPE_DISASSEMBLER("add");
pub const BGEU_DISASSEMBLER: Disassembler = &B_TYPE_DISASSEMBLER("add");
pub const LB_DISASSEMBLER: Disassembler = &LOAD_DISASSEMBLER("lb");
pub const LH_DISASSEMBLER: Disassembler = &LOAD_DISASSEMBLER("lh");
pub const LW_DISASSEMBLER: Disassembler = &LOAD_DISASSEMBLER("lw");
pub const LBU_DISASSEMBLER: Disassembler = &LOAD_DISASSEMBLER("lbu");
pub const LHU_DISASSEMBLER: Disassembler = &LOAD_DISASSEMBLER("lbhu");
pub const SB_DISASSEMBLER: Disassembler = &S_TYPE_DISASSEMBLER("sb");
pub const SH_DISASSEMBLER: Disassembler = &S_TYPE_DISASSEMBLER("sh");
pub const SW_DISASSEMBLER: Disassembler = &S_TYPE_DISASSEMBLER("sw");
pub const ADDI_DISASSEMBLER: Disassembler = &I_TYPE_DISASSEMBLER("addi");
pub const SLTI_DISASSEMBLER: Disassembler = &I_TYPE_DISASSEMBLER("slti");
pub const SLTIU_DISASSEMBLER: Disassembler = &I_TYPE_DISASSEMBLER("sltiu");
pub const XORI_DISASSEMBLER: Disassembler = &I_TYPE_DISASSEMBLER("xori");
pub const ORI_DISASSEMBLER: Disassembler = &I_TYPE_DISASSEMBLER("ori");
pub const ANDI_DISASSEMBLER: Disassembler = &I_TYPE_DISASSEMBLER("andi");
pub const SLLI_DISASSEMBLER: Disassembler = &I_TYPE_DISASSEMBLER("slli");
pub const SRLI_DISASSEMBLER: Disassembler = &I_TYPE_DISASSEMBLER("srli");
pub const SRAI_DISASSEMBLER: Disassembler = &I_TYPE_DISASSEMBLER("srai");
pub const ADD_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("add");
pub const SUB_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("sub");
pub const SLL_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("sll");
pub const SLT_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("slt");
pub const SLTU_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("sltu");
pub const XOR_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("xor");
pub const SRL_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("srl");
pub const SRA_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("sra");
pub const OR_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("or");
pub const AND_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("and");
pub const FENCE_DISASSEMBLER: Disassembler = &SYSTEM_DISASSEMBLER("fence");
pub const FENCE_TSO_DISASSEMBLER: Disassembler = &SYSTEM_DISASSEMBLER("fence_tso");
pub const PAUSE_DISASSEMBLER: Disassembler = &SYSTEM_DISASSEMBLER("pause");
pub const ECALL_DISASSEMBLER: Disassembler = &SYSTEM_DISASSEMBLER("ecall");
pub const EBREAK_DISASSEMBLER: Disassembler = &SYSTEM_DISASSEMBLER("ebreak");
pub const MUL_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("mul");
pub const MULH_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("mulh");
pub const MULHSU_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("mulhsu");
pub const MULHU_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("mulhu");
pub const DIV_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("div");
pub const DIVU_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("divu");
pub const REMU_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("remu");
pub const REM_DISASSEMBLER: Disassembler = &R_TYPE_DISASSEMBLER("rem");

pub fn UNDEF_DISASSEMBLER(instruction: u32, _machine: Option<&Machine>) -> String {
  panic!("UNDEF ({instruction})")
}

const REGISTER_NAMES: [&str; 32] = [
  "x0",
  "x1",
  "x2",
  "x3",
  "x4",
  "x5",
  "x6",
  "x7",
  "x8",
  "x9",
  "x10",
  "x11",
  "x12",
  "x13",
  "x14",
  "x15",
  "x16",
  "x17",
  "x18",
  "x19",
  "x20",
  "x21",
  "x22",
  "x23",
  "x24",
  "x25",
  "x26",
  "x27",
  "x28",
  "x29",
  "x30",
  "x31",
];

fn get_register_name(register: usize) -> String {
  REGISTER_NAMES[register].to_owned()
}
