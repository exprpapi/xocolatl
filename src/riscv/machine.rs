use super::decode::decode;

const MEM_SIZE: usize = 1 << 24;

#[derive(Debug)]
pub struct Memory {
  memory: Vec<u8>,
}

impl Memory {
  pub fn new(size: usize) -> Self {
    Self {
      memory: vec![0; size],
    }
  }

  pub fn load_byte(&self, address: usize) -> u8 {
    self.memory[address]
  }

  pub fn load_halfword(&self, address: usize) -> u16 {
    let lower_byte = self.load_byte(address) as u16;
    let upper_byte = (self.load_byte(address + 1) as u16) << 8;
    lower_byte | upper_byte
  }

  pub fn load_word(&self, address: usize) -> u32 {
    let lower_halfword = self.load_halfword(address) as u32;
    let upper_halfword = (self.load_halfword(address + 2) as u32) << 16;
    lower_halfword | upper_halfword
  }

  pub fn store_byte(&mut self, address: usize, value: u8) {
    self.memory[address] = value;
  }

  pub fn store_halfword(&mut self, address: usize, value: u16) {
    let lower_byte = (value & 0xff) as u8;
    let upper_byte = (value >> 8) as u8;
    self.store_byte(address, lower_byte);
    self.store_byte(address + 1, upper_byte);
  }

  pub fn store_word(&mut self, address: usize, value: u32) {
    let lower_halfword = (value & 0xffff) as u16;
    let upper_halfword = (value >> 16) as u16;
    self.store_halfword(address, lower_halfword);
    self.store_halfword(address + 2, upper_halfword);
  }
}

#[derive(Debug)]
pub struct Registers {
  x: [u32; 32],
}

impl Registers {
  fn new() -> Self {
    Self { x: [0; 32] }
  }

  pub fn get(&self, register: usize) -> u32 {
    match register {
      0 => 0,
      _ => self.x[register],
    }
  }

  pub fn set(&mut self, register: usize, value: u32) {
    if register != 0 {
      self.x[register] = value;
    }
  }
}

#[derive(Debug)]
pub struct Machine {
  pub pc: u32,
  pub registers: Registers,
  pub memory: Memory,
}

impl Default for Machine {
  fn default() -> Self {
    Self {
      pc: 0,
      registers: Registers::new(),
      memory: Memory::new(MEM_SIZE),
    }
  }
}

impl Machine {
  // pub fn new() -> Self {
  //   Self {
  //     pc: 0,
  //     registers: Registers::new(),
  //     memory: Memory::new(MEM_SIZE),
  //   }
  // }

  pub fn cycles(&mut self, cycles: usize) {
    for _ in 0..cycles {
      self.cycle();
    }
  }

  pub fn cycle(&mut self) {
    let instruction = self.fetch();
    self.exec(instruction);
  }

  fn fetch(&self) -> u32 {
    self.memory.load_word(self.pc as usize)
  }

  fn exec(&mut self, instruction: u32) {
    let instruction_bits = instruction;
    let instruction = decode(instruction);
    println!("{}", instruction.disassemble(instruction_bits, Some(self)));
    instruction.exec(self, instruction_bits);
  }
}
