mod opcodes;

use crate::{ CHIP8, WINDOW_W, WINDOW_H, load_file };

use std::io;

use image::{ RgbaImage, Rgba };

// Start-Up, Program loading
impl CHIP8 {
	pub fn new(args: Option<&[String]>) -> io::Result<CHIP8> {
		let mut program_path = None;
		match args {
			Some(a) => {
				if a.len() < 2 {
					return Err(io::Error::new(io::ErrorKind::InvalidInput, "program path missing"));
				} else {
					program_path = Some(&a[1]);
				}
			},
			None => ()
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
		
		match program_path {
			Some(p) => {
				if let Err(e) = c.load_program(&p) {
					return Err(e)
				}
			},
			None => ()
		}

		Ok(c)
	}
	
	fn load_program(&mut self, path: &str) -> io::Result<()> {
		let file = load_file(path)?;
		
		if 4096 - 512 < file.len() {
			return Err(io::Error::new(io::ErrorKind::WriteZero, "ROM too big for memory"));
		}

		for i in 0 .. file.len() {
			self.memory[i + 512] = file[i]
		}

		Ok(())
	}

	// Create an image from the vram
	fn create_screen_image(&self) -> RgbaImage {
		let mut txt = RgbaImage::from_pixel(WINDOW_W.into(), WINDOW_H.into(), 
											Rgba([0, 0, 0, 255]));
		
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

	pub fn handle_input(&mut self, scancode: u32) {
		match scancode {
			0x2D => self.key[0x0] = 1, // X
			0x02 => self.key[0x1] = 1, // 1
			0x03 => self.key[0x2] = 1, // 2
			0x04 => self.key[0x3] = 1, // 3
			0x10 => self.key[0x4] = 1, // Q
			0x11 => self.key[0x5] = 1, // W
			0x12 => self.key[0x6] = 1, // E
			0x1E => self.key[0x7] = 1, // A
			0x1F => self.key[0x8] = 1, // S
			0x20 => self.key[0x9] = 1, // D
			0x2C => self.key[0xA] = 1, // Z
			0x2E => self.key[0xB] = 1, // E
			0x05 => self.key[0xC] = 1, // 4
			0x13 => self.key[0xD] = 1, // R
			0x21 => self.key[0xE] = 1, // F
			0x2F => self.key[0xF] = 1, // V
			_ => (),
		}
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

		println!("opcode: {:X}", self.opcode);

		// Decode opcode
		self.handle_opcode();
		
		self.update_timers();

		if self.draw_flag {
			self.draw_flag = false; // reset the draw flag
			Some(self.create_screen_image())
		} else {
			None
		}
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
