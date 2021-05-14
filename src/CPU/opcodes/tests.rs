#![allow(non_snake_case)]

use crate::CHIP8;

#[test]
fn test_opcode_6XNN() {
	let mut machine = CHIP8::new(None).unwrap();

	machine.memory[machine.pc as usize] = 0x61; // opcode and register id
	machine.memory[machine.pc as usize + 1] = 0x2A; // value

	machine.emulate_cycle();

	assert_eq!(machine.V[1], 0x2A);
}
