#![allow(non_snake_case)]

use crate::CHIP8;

use std::convert::TryInto;

fn get_default_machine(initial_instruction: u16) -> CHIP8 {
	let first_byte: u8 = (initial_instruction >> 8).try_into().unwrap();
	let second_byte: u8 = (initial_instruction & 0x00FF).try_into().unwrap();

	let mut machine = CHIP8::new(None).unwrap();

	machine.memory[machine.pc as usize] = first_byte;
	machine.memory[machine.pc as usize + 1] = second_byte;

	machine
}

#[test]
fn opcode_3XNN_equals() {
	let mut machine = get_default_machine(0x312A);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x2A;

	machine.emulate_cycle();

	assert_eq!(machine.pc, previous_pc + 4);
}

#[test]
fn opcode_3XNN_not_equals() {
	let mut machine = get_default_machine(0x312A);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x2B;

	machine.emulate_cycle();

	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_4XNN_equals() {
	let mut machine = get_default_machine(0x412A);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x2A;

	machine.emulate_cycle();

	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_4XNN_not_equals() {
	let mut machine = get_default_machine(0x412A);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x2B;

	machine.emulate_cycle();

	assert_eq!(machine.pc, previous_pc + 4);
}

#[test]
fn opcode_6XNN() {
	let mut machine = get_default_machine(0x612A);

	let previous_pc = machine.pc;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0x2A);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_7XNN() {
	let mut machine = get_default_machine(0x7129);

	let previous_pc = machine.pc;

	machine.V[0x1] = 1;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0x2A);
	assert_eq!(machine.V[0xF], 0); // check carry flag isn't changed
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_7XNN_overflow() {
	let mut machine = get_default_machine(0x7164);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0xC8;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0x2C);
	assert_eq!(machine.V[0xF], 0); // check carry flag isn't changed
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY0() {
	let mut machine = get_default_machine(0x8120);

	let previous_pc = machine.pc;

	machine.V[0x2] = 0x2A;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0x2A);
	assert_eq!(machine.V[0x2], 0x2A);
	assert_eq!(machine.pc, previous_pc + 2);
}
