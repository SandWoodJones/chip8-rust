mod opcodes;

use crate::{ CHIP8, WINDOW_W, WINDOW_H };

use std::fs;
use std::io::{self, Read};

use image::{ RgbaImage, Rgba };

// Start-Up, Program loading
impl CHIP8 {
	pub fn new(args: &[String]) -> io::Result<CHIP8> {
		if args.len() < 2 {
			return Err(io::Error::new(io::ErrorKind::InvalidInput, "program path missing"));
		}

		let mut c = CHIP8 {
			opcode: 0x0000, 
			memory: [0x00u8; 4096],
			V: [0x00; 16],
			I: 0x0000,
			pc: 0x0200, // Program counter starts at 512
			gfx: [0; WINDOW_W as usize * WINDOW_H as usize],
			draw_flag: false,
			delay_timer: 0x00,
			sound_timer: 0x00,
			stack: [0x0000; 16],
			sp: 0x0000,
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

	// Create an image from the vram
	fn create_screen_image(&self) -> RgbaImage {
		let mut txt = RgbaImage::from_pixel(WINDOW_W.into(), WINDOW_H.into(), Rgba([0, 0, 0, 255]));
		
		for y in 0 .. WINDOW_H {
			for x in 0 .. WINDOW_W {
				if self.gfx[(y as usize * WINDOW_W as usize) + x as usize] == 0 {
					txt.put_pixel(x.into(), y.into(), Rgba([0, 0, 0, 255])); // disabled
				} else {
					txt.put_pixel(x.into(), y.into(), Rgba([255, 255, 255, 255])); // enabled
				}
			}
		}

		txt
	}
}

// Emulating
impl CHIP8 {
	// Emulates one cycle of the CPU
	pub fn emulate_cycle(&mut self) -> Option<RgbaImage> {
		// Fetch opcode
		let opc1 = self.memory[self.pc as usize] as u16; // First byte 
		let opc2 = self.memory[(self.pc + 1) as usize] as u16; // Second byte
		// Merge the 2 bytes, by shifting the first by 8 and ORing the second.
		self.opcode = opc1 << 8 | opc2;

		//println!("opcode: {:X}", self.opcode);

		// Decode opcode
		self.handle_opcode(self.opcode);
		
		let screen;
		if self.draw_flag { // the draw flag has been set
			screen = Some(self.create_screen_image());
		} else { screen = None; }

		self.draw_flag = false; // reset the draw flag

		self.update_timers();
		screen
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
