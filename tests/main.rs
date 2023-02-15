use xocolatl::riscv::{disassemble::disassemble, assemble::assemble_normalized};

#[test]
fn test_disassemble() {
  const TEST_CASES: &[(u32, &str)] = &[
    (0x00b604a3, "sb x11, 9(x12)"),
    (0x07b00513, "addi x10, x0, 123"),
    (0x00000073, "ecall"),
    (0x00a5a603, "lw x12, 10(x11)"),
    (0x004db097, "auipc x1, 1243"),
    (0x0007b4b7, "lui x9, 123"),
    (0x00c40267, "jalr x4, 12(x8)"),
    (0x0a400f6f, "jal x30, 164"),
  ];

  for (assembly, disassembly) in TEST_CASES {
    assert_eq!(disassemble(*assembly), *disassembly);
  }
}

#[test]
fn test_assemble_normalized() {
  const TEST_CASES: &[(u32, &str)] = &[
    (0x00b604a3, "sb x11 x12 9"),
    (0x07b00513, "addi x10 x0 123"),
    (0x00000073, "ecall"),
    (0x00a5a603, "lw x12 x11 10"),
    (0x004db097, "auipc x1 1243"),
    (0x0007b4b7, "lui x9 123"),
    (0x00c40267, "jalr x4 x8 12"),
    (0x0a400f6f, "jal x30 164"),
  ];

  for test_case in TEST_CASES {
    let (assembly, disassembly) = test_case;
    let left = *assembly;
    let right = assemble_normalized(disassembly);
    let left = format!("{left:#034b}");
    let right = format!("{right:#034b}");
    assert_eq!(left, right, "failed {test_case:?}");
  }
}