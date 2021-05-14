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
fn opcode_00E0() {
	let mut machine = get_default_machine(0x00E0);

	let previous_pc = machine.pc;

	machine.gfx = [1; 2048]; // enable all pixels

	machine.emulate_cycle();
	
	for i in 0 .. 2048 {
		assert_eq!(machine.gfx[i], 0);
	}
	assert_eq!(machine.draw_flag, true);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_1NNN() {
	let mut machine = get_default_machine(0x129A);

	machine.emulate_cycle();

	assert_eq!(machine.pc, 0x29A);
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
fn opcode_5XY0_equals() {
	let mut machine = get_default_machine(0x5120);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x2A;
	machine.V[0x2] = 0x2A;

	machine.emulate_cycle();

	assert_eq!(machine.pc, previous_pc + 4);
}

#[test]
fn opcode_5XY0_not_equals() {
	let mut machine = get_default_machine(0x5120);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x2A;
	machine.V[0x2] = 0x2B;

	machine.emulate_cycle();

	assert_eq!(machine.pc, previous_pc + 2);
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
fn opcode_7XNN_with_overflow() {
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

#[test]
fn opcode_8XY1() {
	let mut machine = get_default_machine(0x8121);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0b1010;
	machine.V[0x2] = 0b0101;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0b1111);
	assert_eq!(machine.V[0x2], 0b0101);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY2() {
	let mut machine = get_default_machine(0x8122);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0b1011;
	machine.V[0x2] = 0b0101;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0b0001);
	assert_eq!(machine.V[0x2], 0b0101);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY3() {
	let mut machine = get_default_machine(0x8123);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0b1111;
	machine.V[0x2] = 0b1001;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0b0110);
	assert_eq!(machine.V[0x2], 0b1001);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY4() {
	let mut machine = get_default_machine(0x8124);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x3C;
	machine.V[0x2] = 0x9;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0x45);
	assert_eq!(machine.V[0x2], 0x9);
	assert_eq!(machine.V[0xF], 0x0);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY4_with_overflow() {
	let mut machine = get_default_machine(0x8124);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0xFF;
	machine.V[0x2] = 0xF;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0xE);
	assert_eq!(machine.V[0x2], 0xF);
	assert_eq!(machine.V[0xF], 0x1);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY5() {
	let mut machine = get_default_machine(0x8125);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x89;
	machine.V[0x2] = 0x5F;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0x2A);
	assert_eq!(machine.V[0x2], 0x5F);
	assert_eq!(machine.V[0xF], 0x0);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY5_with_overflow() {
	let mut machine = get_default_machine(0x8125);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x89;
	machine.V[0x2] = 0x8A;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0xFF);
	assert_eq!(machine.V[0x2], 0x8A);
	assert_eq!(machine.V[0xF], 0x1);
	assert_eq!(machine.pc, previous_pc + 2);
}
