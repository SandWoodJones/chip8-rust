mod opcodes;

use crate::CHIP8;

use std::fs;
use std::io::{self, Read};

// Start-Up, Program loading
impl CHIP8 {
	pub fn new(args: &[String]) -> io::Result<CHIP8> {
		if args.len() < 2 {
			return Err(io::Error::new(io::ErrorKind::InvalidInput, "program path missing"));
		}

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
		for i in 0 .. 80 {
			c.memory[i] = crate::FONTSET[i];
		}

		if let Err(e) = c.load_program(&args[1]) {
			return Err(e)
		}

		Ok(c)
	}
	
	fn load_program(&mut self, path: &str) -> io::Result<()> {
		let file = CHIP8::load_file(path)?;

		if 4096 - 512 < file.len() {
			return Err(io::Error::new(io::ErrorKind::WriteZero, "ROM too big for memory"));
		}

		for i in 0 .. file.len() {
			self.memory[i + 512] = file[i]
		}

		Ok(())
	}

	// Read a file in binary mode and store it in a buffer
	fn load_file(path: &str) -> io::Result<Vec<u8>> {
		let mut p = fs::File::open(path)?;
		let mut buf = Vec::new();

		p.read_to_end(&mut buf)?;
		Ok(buf)
	}
}

// Emulating
impl CHIP8 {
	// Emulates one cycle of the CPU
	pub fn emulate_cycle(&mut self) {
		// Fetch opcode
		let opc1 = self.memory[self.pc as usize]; // First byte 
		let opc2 = self.memory[(self.pc + 1) as usize]; // Second byte
		// Merge the 2 bytes, by shifting the first by 8 and ORing the second.
		self.opcode = (opc1 as u16) << 8 | opc2 as u16;

		// Decode opcode
		self.handle_opcode(self.opcode);

		self.update_timers();
	}

	fn update_timers(&mut self) {
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
