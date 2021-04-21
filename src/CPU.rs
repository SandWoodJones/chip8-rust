mod opcodes;
mod fontset;

use crate::CPU::opcodes::handle_opcode;
use crate::CPU::fontset::get_fontset;

use std::fs::File;
use std::io::{self, Read, Error, ErrorKind};

pub struct CHIP8 {
	opcode: u16, // current opcode
	memory: [u8; 4096],
	V: [u8; 16], // cpu registers
	I: u16, // index register
	pc: u16, // program counter
	gfx: [u8; 64 * 32], // visual ram
	delay_timer: u8,
	sound_timer: u8,
	stack: [u16; 16],
	sp: u16, // stack pointer
	key: [u8; 16]
}

impl CHIP8 {
	pub fn new() -> CHIP8 {
		let mut c = CHIP8 {
			opcode: 0x0000, // Reset current opcode
			memory: [0x00u8; 4096],
			V: [0x00; 16],
			I: 0x0000, // Reset index register
			pc: 0x0200, // Program counter starts at 0x200
			gfx: [0; 64 * 32],
			delay_timer: 0x00,
			sound_timer: 0x00,
			stack: [0x0000; 16],
			sp: 0x0000, // Reset stack pointer
			key: [0x00; 16]
		};

		// load fontset into memory
		let fontset = get_fontset();
		for i in 0 .. 80 {
			c.memory[i] = fontset[i];
		}
		c
	}
	
	pub fn load_program(&mut self, path: &str) -> io::Result<()> {
		let f = CHIP8::load_file(path)?;
		if 4096 - 512 < f.len() {
			return Err(Error::new(ErrorKind::WriteZero, "ROM too big for memory"));
		}
		for i in 0 .. f.len() {
			self.memory[i + 512] = f[i]
		}
		Ok(())
	}

	// Read a binary file and store it in a buffer
	fn load_file(path: &str) -> io::Result<Vec<u8>> {
		let mut p = File::open(path)?;
		let mut buf = Vec::new();
		p.read_to_end(&mut buf)?;
		Ok(buf)
	}

	// Emulates one cycle of the CPU
	pub fn emulate_cycle(&mut self) {
		// Fetch opcode
		let opc1 = self.memory[self.pc as usize]; // First byte 
		let opc2 = self.memory[(self.pc + 1) as usize]; // Second byte
		// Merge the 2 bytes, by shifting the first by 8 and ORing the second.
		self.opcode = (opc1 as u16) << 8 | opc2 as u16;

		// Decode opcode
		handle_opcode(self.opcode, self);

		// Update timers
		if self.delay_timer > 0 { // if timer is above zero, count down to zero
			self.delay_timer -= 1;
		}
		if self.sound_timer > 0 { // does the same but also beeps when it gets to 1
			if self.sound_timer == 1 {
				println!("BEEP!");
			}
			self.sound_timer -= 1;
		}
	}
}

