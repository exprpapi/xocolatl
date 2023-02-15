use super::machine::Machine; 
use super::executors::*;
use super::assemblers::*;
use super::disassemblers::*;

pub enum InstructionIndex {
  LUI,
  AUIPC,
  JAL,
  JALR,
  BEQ,
  BNE,
  BLT,
  BGE,
  BLTU,
  BGEU,
  LB,
  LH,
  LW,
  LBU,
  LHU,
  SB,
  SH,
  SW,
  ADDI,
  SLTI,
  SLTIU,
  XORI,
  ORI,
  ANDI,
  SLLI,
  SRLI,
  SRAI,
  ADD,
  SUB,
  SLL,
  SLT,
  SLTU,
  XOR,
  SRL,
  SRA,
  OR,
  AND,
  FENCE,
  FENCE_TSO,
  PAUSE,
  ECALL,
  EBREAK,
  MUL,
  MULH,
  MULHSU,
  MULHU,
  DIV,
  DIVU,
  REM,
  REMU,
  UNDEF,
}

pub const INSTRUCTION_COUNT: usize = std::mem::variant_count::<InstructionIndex>();

pub struct Instruction {
  pub name: &'static str,
  pub executor: Executor,
  pub assembler: Assembler,
  pub disassembler: Disassembler,
}

impl Instruction {
  pub fn exec(&self, machine: &mut Machine, instruction: u32) {
    println!("Executing {}", self.name);
    (self.executor)(machine, instruction);
  }

  pub fn disassemble(&self, instruction: u32, machine: Option<&Machine>) -> String {
    (self.disassembler)(instruction, machine)
  }
}

pub const INSTRUCTION_LUI: Instruction = Instruction {
  name: "lui",
  executor: &LUI_EXECUTOR,
  disassembler: &LUI_DISASSEMBLER,
  assembler: &LUI_ASSEMBLER,
};

pub const INSTRUCTION_AUIPC: Instruction = Instruction {
  name: "auipc",
  executor: &AUIPC_EXECUTOR,
  disassembler: &AUIPC_DISASSEMBLER,
  assembler: &AUIPC_ASSEMBLER,
};

pub const INSTRUCTION_JAL: Instruction = Instruction {
  name: "jal",
  executor: &JAL_EXECUTOR,
  disassembler: &JAL_DISASSEMBLER,
  assembler: &JAL_ASSEMBLER,
};

pub const INSTRUCTION_JALR: Instruction = Instruction {
  name: "jalr",
  executor: &JALR_EXECUTOR,
  disassembler: &JALR_DISASSEMBLER,
  assembler: &JALR_ASSEMBLER,
};

pub const INSTRUCTION_BEQ: Instruction = Instruction {
  name: "beq",
  executor: &BEQ_EXECUTOR,
  disassembler: &BEQ_DISASSEMBLER,
  assembler: &BEQ_ASSEMBLER,
};

pub const INSTRUCTION_BNE: Instruction = Instruction {
  name: "bne",
  executor: &BNE_EXECUTOR,
  disassembler: &BNE_DISASSEMBLER,
  assembler: &BNE_ASSEMBLER,
};

pub const INSTRUCTION_BLT: Instruction = Instruction {
  name: "blt",
  executor: &BLT_EXECUTOR,
  disassembler: &BLT_DISASSEMBLER,
  assembler: &BLT_ASSEMBLER,
};

pub const INSTRUCTION_BGE: Instruction = Instruction {
  name: "bge",
  executor: &BGE_EXECUTOR,
  disassembler: &BGE_DISASSEMBLER,
  assembler: &BGE_ASSEMBLER,
};

pub const INSTRUCTION_BLTU: Instruction = Instruction {
  name: "bltu",
  executor: &BLTU_EXECUTOR,
  disassembler: &BLTU_DISASSEMBLER,
  assembler: &BLTU_ASSEMBLER,
};

pub const INSTRUCTION_BGEU: Instruction = Instruction {
  name: "bgeu",
  executor: &BGEU_EXECUTOR,
  disassembler: &BGEU_DISASSEMBLER,
  assembler: &BGEU_ASSEMBLER,
};

pub const INSTRUCTION_LB: Instruction = Instruction {
  name: "lb",
  executor: &LB_EXECUTOR,
  disassembler: &LB_DISASSEMBLER,
  assembler: &LB_ASSEMBLER,
};

pub const INSTRUCTION_LH: Instruction = Instruction {
  name: "lh",
  executor: &LH_EXECUTOR,
  disassembler: &LH_DISASSEMBLER,
  assembler: &LH_ASSEMBLER,
};

pub const INSTRUCTION_LW: Instruction = Instruction {
  name: "lw",
  executor: &LW_EXECUTOR,
  disassembler: &LW_DISASSEMBLER,
  assembler: &LW_ASSEMBLER,
};

pub const INSTRUCTION_LBU: Instruction = Instruction {
  name: "lbu",
  executor: &LBU_EXECUTOR,
  disassembler: &LBU_DISASSEMBLER,
  assembler: &LBU_ASSEMBLER,
};

pub const INSTRUCTION_LHU: Instruction = Instruction {
  name: "lhu",
  executor: &LHU_EXECUTOR,
  disassembler: &LHU_DISASSEMBLER,
  assembler: &LHU_ASSEMBLER,
};

pub const INSTRUCTION_SB: Instruction = Instruction {
  name: "sb",
  executor: &SB_EXECUTOR,
  disassembler: &SB_DISASSEMBLER,
  assembler: &SB_ASSEMBLER,
};

pub const INSTRUCTION_SH: Instruction = Instruction {
  name: "sh",
  executor: &SH_EXECUTOR,
  disassembler: &SH_DISASSEMBLER,
  assembler: &SH_ASSEMBLER,
};

pub const INSTRUCTION_SW: Instruction = Instruction {
  name: "sw",
  executor: &SW_EXECUTOR,
  disassembler: &SW_DISASSEMBLER,
  assembler: &SW_ASSEMBLER,
};

pub const INSTRUCTION_ADDI: Instruction = Instruction {
  name: "addi",
  executor: &ADDI_EXECUTOR,
  disassembler: &ADDI_DISASSEMBLER,
  assembler: &ADDI_ASSEMBLER,
};

pub const INSTRUCTION_SLTI: Instruction = Instruction {
  name: "slti",
  executor: &SLTI_EXECUTOR,
  disassembler: &SLTI_DISASSEMBLER,
  assembler: &SLTI_ASSEMBLER,
};

pub const INSTRUCTION_SLTIU: Instruction = Instruction {
  name: "sltiu",
  executor: &SLTIU_EXECUTOR,
  disassembler: &SLTIU_DISASSEMBLER,
  assembler: &SLTIU_ASSEMBLER,
};

pub const INSTRUCTION_XORI: Instruction = Instruction {
  name: "xori",
  executor: &XORI_EXECUTOR,
  disassembler: &XORI_DISASSEMBLER,
  assembler: &XORI_ASSEMBLER,
};

pub const INSTRUCTION_ORI: Instruction = Instruction {
  name: "ori",
  executor: &ORI_EXECUTOR,
  disassembler: &ORI_DISASSEMBLER,
  assembler: &ORI_ASSEMBLER,
};

pub const INSTRUCTION_ANDI: Instruction = Instruction {
  name: "andi",
  executor: &ANDI_EXECUTOR,
  disassembler: &ANDI_DISASSEMBLER,
  assembler: &ANDI_ASSEMBLER,
};

pub const INSTRUCTION_SLLI: Instruction = Instruction {
  name: "slli",
  executor: &SLLI_EXECUTOR,
  disassembler: &SLLI_DISASSEMBLER,
  assembler: &SLLI_ASSEMBLER,
};

pub const INSTRUCTION_SRLI: Instruction = Instruction {
  name: "srli",
  executor: &SRLI_EXECUTOR,
  disassembler: &SRLI_DISASSEMBLER,
  assembler: &SRLI_ASSEMBLER,
};

pub const INSTRUCTION_SRAI: Instruction = Instruction {
  name: "srai",
  executor: &SRAI_EXECUTOR,
  disassembler: &SRAI_DISASSEMBLER,
  assembler: &SRAI_ASSEMBLER,
};

pub const INSTRUCTION_ADD: Instruction = Instruction {
  name: "add",
  executor: &ADD_EXECUTOR,
  disassembler: &ADD_DISASSEMBLER,
  assembler: &ADD_ASSEMBLER,
};

pub const INSTRUCTION_SUB: Instruction = Instruction {
  name: "sub",
  executor: &SUB_EXECUTOR,
  disassembler: &SUB_DISASSEMBLER,
  assembler: &SUB_ASSEMBLER,
};

pub const INSTRUCTION_SLL: Instruction = Instruction {
  name: "sll",
  executor: &SLL_EXECUTOR,
  disassembler: &SLL_DISASSEMBLER,
  assembler: &SLL_ASSEMBLER,
};

pub const INSTRUCTION_SLT: Instruction = Instruction {
  name: "slt",
  executor: &SLT_EXECUTOR,
  disassembler: &SLT_DISASSEMBLER,
  assembler: &SLT_ASSEMBLER,
};

pub const INSTRUCTION_SLTU: Instruction = Instruction {
  name: "sltu",
  executor: &SLTU_EXECUTOR,
  disassembler: &SLTU_DISASSEMBLER,
  assembler: &SLTU_ASSEMBLER,
};

pub const INSTRUCTION_XOR: Instruction = Instruction {
  name: "xor",
  executor: &XOR_EXECUTOR,
  disassembler: &XOR_DISASSEMBLER,
  assembler: &XOR_ASSEMBLER,
};

pub const INSTRUCTION_SRL: Instruction = Instruction {
  name: "srl",
  executor: &SRL_EXECUTOR,
  disassembler: &SRL_DISASSEMBLER,
  assembler: &SRL_ASSEMBLER,
};

pub const INSTRUCTION_SRA: Instruction = Instruction {
  name: "sra",
  executor: &SRA_EXECUTOR,
  disassembler: &SRA_DISASSEMBLER,
  assembler: &SRA_ASSEMBLER,
};

pub const INSTRUCTION_OR: Instruction = Instruction {
  name: "or",
  executor: &OR_EXECUTOR,
  disassembler: &OR_DISASSEMBLER,
  assembler: &OR_ASSEMBLER,
};

pub const INSTRUCTION_AND: Instruction = Instruction {
  name: "and",
  executor: &AND_EXECUTOR,
  disassembler: &AND_DISASSEMBLER,
  assembler: &AND_ASSEMBLER,
};

pub const INSTRUCTION_FENCE: Instruction = Instruction {
  name: "fence",
  executor: &FENCE_EXECUTOR,
  disassembler: &FENCE_DISASSEMBLER,
  assembler: &FENCE_ASSEMBLER,
};

pub const INSTRUCTION_FENCE_TSO: Instruction = Instruction {
  name: "fence_tso",
  executor: &FENCE_TSO_EXECUTOR,
  disassembler: &FENCE_TSO_DISASSEMBLER,
  assembler: &FENCE_TSO_ASSEMBLER,
};

pub const INSTRUCTION_PAUSE: Instruction = Instruction {
  name: "pause",
  executor: &PAUSE_EXECUTOR,
  disassembler: &PAUSE_DISASSEMBLER,
  assembler: &PAUSE_ASSEMBLER,
};

pub const INSTRUCTION_ECALL: Instruction = Instruction {
  name: "ecall",
  executor: &ECALL_EXECUTOR,
  disassembler: &ECALL_DISASSEMBLER,
  assembler: &ECALL_ASSEMBLER,
};

pub const INSTRUCTION_EBREAK: Instruction = Instruction {
  name: "ebreak",
  executor: &EBREAK_EXECUTOR,
  disassembler: &EBREAK_DISASSEMBLER,
  assembler: &EBREAK_ASSEMBLER,
};

pub const INSTRUCTION_MUL: Instruction = Instruction {
  name: "mul",
  executor: &MUL_EXECUTOR,
  disassembler: &MUL_DISASSEMBLER,
  assembler: &MUL_ASSEMBLER,
};

pub const INSTRUCTION_MULH: Instruction = Instruction {
  name: "mulh",
  executor: &MULH_EXECUTOR,
  disassembler: &MULH_DISASSEMBLER,
  assembler: &MULH_ASSEMBLER,
};

pub const INSTRUCTION_MULHSU: Instruction = Instruction {
  name: "mulhsu",
  executor: &MULHSU_EXECUTOR,
  disassembler: &MULHSU_DISASSEMBLER,
  assembler: &MULHSU_ASSEMBLER,
};

pub const INSTRUCTION_MULHU: Instruction = Instruction {
  name: "mulhu",
  executor: &MULHU_EXECUTOR,
  disassembler: &MULHU_DISASSEMBLER,
  assembler: &MULHU_ASSEMBLER,
};

pub const INSTRUCTION_DIV: Instruction = Instruction {
  name: "div",
  executor: &DIV_EXECUTOR,
  disassembler: &DIV_DISASSEMBLER,
  assembler: &DIV_ASSEMBLER,
};

pub const INSTRUCTION_DIVU: Instruction = Instruction {
  name: "divu",
  executor: &DIVU_EXECUTOR,
  disassembler: &DIVU_DISASSEMBLER,
  assembler: &DIVU_ASSEMBLER,
};

pub const INSTRUCTION_REM: Instruction = Instruction {
  name: "rem",
  executor: &REM_EXECUTOR,
  disassembler: &REM_DISASSEMBLER,
  assembler: &REM_ASSEMBLER,
};

pub const INSTRUCTION_REMU: Instruction = Instruction {
  name: "remu",
  executor: &REMU_EXECUTOR,
  disassembler: &REMU_DISASSEMBLER,
  assembler: &REMU_ASSEMBLER,
};

/// sentinel instruction; panics
pub const INSTRUCTION_UNDEF: Instruction = Instruction {
  name: "undef",
  executor: &UNDEF_EXECUTOR,
  disassembler: &UNDEF_DISASSEMBLER,
  assembler: &UNDEF_ASSEMBLER,
};

pub const INSTRUCTIONS: [Instruction; INSTRUCTION_COUNT] = {
  let mut instructions = [INSTRUCTION_UNDEF; INSTRUCTION_COUNT];
  use InstructionIndex::*;
  instructions[LUI as usize] = INSTRUCTION_LUI;
  instructions[AUIPC as usize] = INSTRUCTION_AUIPC;
  instructions[JAL as usize] = INSTRUCTION_JAL;
  instructions[JALR as usize] = INSTRUCTION_JALR;
  instructions[BEQ as usize] = INSTRUCTION_BEQ;
  instructions[BNE as usize] = INSTRUCTION_BNE;
  instructions[BLT as usize] = INSTRUCTION_BLT;
  instructions[BGE as usize] = INSTRUCTION_BGE;
  instructions[BLTU as usize] = INSTRUCTION_BLTU;
  instructions[BGEU as usize] = INSTRUCTION_BGEU;
  instructions[LB as usize] = INSTRUCTION_LB;
  instructions[LH as usize] = INSTRUCTION_LH;
  instructions[LW as usize] = INSTRUCTION_LW;
  instructions[LBU as usize] = INSTRUCTION_LBU;
  instructions[LHU as usize] = INSTRUCTION_LHU;
  instructions[SB as usize] = INSTRUCTION_SB;
  instructions[SH as usize] = INSTRUCTION_SH;
  instructions[SW as usize] = INSTRUCTION_SW;
  instructions[ADDI as usize] = INSTRUCTION_ADDI;
  instructions[SLTI as usize] = INSTRUCTION_SLTI;
  instructions[SLTIU as usize] = INSTRUCTION_SLTIU;
  instructions[XORI as usize] = INSTRUCTION_XORI;
  instructions[ORI as usize] = INSTRUCTION_ORI;
  instructions[ANDI as usize] = INSTRUCTION_ANDI;
  instructions[SLLI as usize] = INSTRUCTION_SLLI;
  instructions[SRLI as usize] = INSTRUCTION_SRLI;
  instructions[SRAI as usize] = INSTRUCTION_SRAI;
  instructions[ADD as usize] = INSTRUCTION_ADD;
  instructions[SUB as usize] = INSTRUCTION_SUB;
  instructions[SLL as usize] = INSTRUCTION_SLL;
  instructions[SLT as usize] = INSTRUCTION_SLT;
  instructions[SLTU as usize] = INSTRUCTION_SLTU;
  instructions[XOR as usize] = INSTRUCTION_XOR;
  instructions[SRL as usize] = INSTRUCTION_SRL;
  instructions[SRA as usize] = INSTRUCTION_SRA;
  instructions[OR as usize] = INSTRUCTION_OR;
  instructions[AND as usize] = INSTRUCTION_AND;
  instructions[FENCE as usize] = INSTRUCTION_FENCE;
  instructions[FENCE_TSO as usize] = INSTRUCTION_FENCE_TSO;
  instructions[PAUSE as usize] = INSTRUCTION_PAUSE;
  instructions[ECALL as usize] = INSTRUCTION_ECALL;
  instructions[EBREAK as usize] = INSTRUCTION_EBREAK;
  instructions[MUL as usize] = INSTRUCTION_MUL;
  instructions[MULH as usize] = INSTRUCTION_MULH;
  instructions[MULHSU as usize] = INSTRUCTION_MULHSU;
  instructions[MULHU as usize] = INSTRUCTION_MULHU;
  instructions[DIV as usize] = INSTRUCTION_DIV;
  instructions[DIVU as usize] = INSTRUCTION_DIVU;
  instructions[REM as usize] = INSTRUCTION_REM;
  instructions[REMU as usize] = INSTRUCTION_REMU;

  instructions
};