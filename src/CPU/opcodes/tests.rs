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
fn opcode_00EE() {
	let mut machine = get_default_machine(0x00EE);

	machine.sp = 11;
	machine.stack[machine.sp as usize - 1] = 40;

	machine.emulate_cycle();

	assert_eq!(machine.sp, 10);
	assert_eq!(machine.pc, 42);
}

#[test]
#[should_panic(expected = "Stack pointer 65535 is out of bounds")]
fn opcode_00EE_out_of_bounds() {
	let mut machine = get_default_machine(0x00EE);

	machine.emulate_cycle();
}

#[test]
fn opcode_1NNN() {
	let mut machine = get_default_machine(0x129A);

	machine.emulate_cycle();

	assert_eq!(machine.pc, 0x29A);
}

#[test]
fn opcode_2NNN() {
	let mut machine = get_default_machine(0x2321);

	let previous_pc = machine.pc;

	machine.emulate_cycle();
	
	assert_eq!(machine.stack[machine.sp as usize - 1], previous_pc);
	assert_eq!(machine.sp, 1);
	assert_eq!(machine.pc, 0x321);
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
	assert_eq!(machine.V[0xF], 0x1);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY5_with_borrow() {
	let mut machine = get_default_machine(0x8125);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x00;
	machine.V[0x2] = 0xFF;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0x1);
	assert_eq!(machine.V[0x2], 0xFF);
	assert_eq!(machine.V[0xF], 0x0);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY6_lsb_set() {
	let mut machine = get_default_machine(0x8126);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0b101;

	machine.emulate_cycle();
	
	assert_eq!(machine.V[0xF], 0b1);
	assert_eq!(machine.V[0x1], 0b10);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY6_lsb_unset() {
	let mut machine = get_default_machine(0x8126);

	let previous_pc = machine.pc;

	machine.emulate_cycle();
	
	assert_eq!(machine.V[0xF], 0b0);
	assert_eq!(machine.V[0x1], 0b0);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY7() {
	let mut machine = get_default_machine(0x8127);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x11;
	machine.V[0x2] = 0xAA;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0x99);
	assert_eq!(machine.V[0xF], 1);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XY7_with_borrow() {
	let mut machine = get_default_machine(0x8127);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0x10;
	machine.V[0x2] = 0xA;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0xFA);
	assert_eq!(machine.V[0xF], 0);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XYE_msb_set() {
	let mut machine = get_default_machine(0x812E);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0b10001001;

	machine.emulate_cycle();

	assert_eq!(machine.V[0xF], 0b1);
	assert_eq!(machine.V[0x1], 0b10010);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_8XYE_msb_unset() {
	let mut machine = get_default_machine(0x812E);

	let previous_pc = machine.pc;

	machine.V[0x1] = 0b00001001;

	machine.emulate_cycle();

	assert_eq!(machine.V[0xF], 0b0);
	assert_eq!(machine.V[0x1], 0b10010);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_9XY0_equals() {
	let mut machine = get_default_machine(0x9230);

	let previous_pc = machine.pc;
	
	machine.V[2] = 0x2A;
	machine.V[3] = 0x2A;

	machine.emulate_cycle();
	
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_9XY0_not_equals() {
	let mut machine = get_default_machine(0x9230);

	let previous_pc = machine.pc;
	
	machine.V[2] = 0x2A;
	machine.V[3] = 0x29;

	machine.emulate_cycle();
	
	assert_eq!(machine.pc, previous_pc + 4);
}

#[test]
fn opcode_ANNN() {
	let mut machine = get_default_machine(0xA123);

	let previous_pc = machine.pc;

	machine.emulate_cycle();
	
	assert_eq!(machine.I, 0x123);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_BNNN() {
	let mut machine = get_default_machine(0xB002);

	machine.V[0x0] = 0x28;

	machine.emulate_cycle();

	assert_eq!(machine.pc, 0x2A);
}

#[test]
fn opcode_FX07() {
	let mut machine = get_default_machine(0xF107);

	let previous_pc = machine.pc;

	machine.delay_timer = 0x2A;

	machine.emulate_cycle();

	assert_eq!(machine.V[0x1], 0x2A);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_FX15() {
	let mut machine = get_default_machine(0xF015);

	let previous_pc = machine.pc;

	machine.V[0x0] = 0x2A;

	machine.emulate_cycle();

	assert_eq!(machine.delay_timer, 0x29);
	assert_eq!(machine.V[0x0], 0x2A);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_FX18() {
	let mut machine = get_default_machine(0xF018);

	let previous_pc = machine.pc;

	machine.V[0x0] = 0x2A;

	machine.emulate_cycle();

	assert_eq!(machine.sound_timer, 0x29);
	assert_eq!(machine.V[0x0], 0x2A);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_FX1E() {
	let mut machine = get_default_machine(0xF31E);

	let previous_pc = machine.pc;

	machine.V[0x3] = 0x2A;
	machine.I = 0x18;

	machine.emulate_cycle();

	assert_eq!(machine.I, 0x42);
	assert_eq!(machine.V[0x3], 0x2A);
	assert_eq!(machine.V[0xF], 0x0);
	assert_eq!(machine.pc, previous_pc + 2);
}

#[test]
fn opcode_FX1E_with_overflow() {
	let mut machine = get_default_machine(0xF31E);

	let previous_pc = machine.pc;

	machine.V[0x3] = 0x32;
	machine.I = 0xFFFE;

	machine.emulate_cycle();

	assert_eq!(machine.I, 0x30);
	assert_eq!(machine.V[0x3], 0x32);
	assert_eq!(machine.V[0xF], 0x0);
	assert_eq!(machine.pc, previous_pc + 2);
}
