mod opcodes;

use crate::{ CHIP8, WINDOW_W, WINDOW_H, load_file };

use std::io;

// Start-Up, program loading and screen updating
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
			draw_flag: true, // Clear screen once
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

}

// Emulating
impl CHIP8 {
	// Emulates one cycle of the CPU
	pub fn emulate_cycle(&mut self) {
		// Fetch opcode
		let opc1 = self.memory[self.pc as usize] as u16; // First byte 
		let opc2 = self.memory[(self.pc + 1) as usize] as u16; // Second byte
		// Merge the 2 bytes, by shifting the first by 8 and ORing the second.
		self.opcode = opc1 << 8 | opc2;

		println!("opcode: {:X}", self.opcode);
		//println!("{:?}", self.key);

		// Decode opcode
		self.handle_opcode();
		
		self.update_timers();
	}

	fn update_timers(&mut self) {
		if self.delay_timer > 0 { // if timer is above zero, count down to zero
			self.delay_timer -= 1;
		}

		if self.sound_timer > 0 { // does the same but also calls play_snd() when it gets to 1
			if self.sound_timer == 1 {
				CHIP8::play_snd();
			}
			self.sound_timer -= 1;
		}
	}
}

use crow::glutin::event::{ KeyboardInput, ElementState };
use image::{ RgbaImage, Rgba };

// Peripherals, input, display, sound
impl CHIP8 {
	pub fn handle_input(&mut self, key: KeyboardInput) {
		match key.scancode {
			0x2D => self.key[0x0] = (key.state == ElementState::Pressed) as u8, // X
			0x02 => self.key[0x1] = (key.state == ElementState::Pressed) as u8, // 1
			0x03 => self.key[0x2] = (key.state == ElementState::Pressed) as u8, // 2
			0x04 => self.key[0x3] = (key.state == ElementState::Pressed) as u8, // 3
			0x10 => self.key[0x4] = (key.state == ElementState::Pressed) as u8, // Q
			0x11 => self.key[0x5] = (key.state == ElementState::Pressed) as u8, // W
			0x12 => self.key[0x6] = (key.state == ElementState::Pressed) as u8, // E
			0x1E => self.key[0x7] = (key.state == ElementState::Pressed) as u8, // A
			0x1F => self.key[0x8] = (key.state == ElementState::Pressed) as u8, // S
			0x20 => self.key[0x9] = (key.state == ElementState::Pressed) as u8, // D
			0x2C => self.key[0xA] = (key.state == ElementState::Pressed) as u8, // Z
			0x2E => self.key[0xB] = (key.state == ElementState::Pressed) as u8, // E
			0x05 => self.key[0xC] = (key.state == ElementState::Pressed) as u8, // 4
			0x13 => self.key[0xD] = (key.state == ElementState::Pressed) as u8, // R
			0x21 => self.key[0xE] = (key.state == ElementState::Pressed) as u8, // F
			0x2F => self.key[0xF] = (key.state == ElementState::Pressed) as u8, // V
			_ => (),
		}
	}

	// Create an image from the vram
	pub fn create_screen_image(&mut self) -> RgbaImage {
		let mut img = RgbaImage::from_pixel(WINDOW_W.into(), WINDOW_H.into(), 
											Rgba([0, 0, 0, 255]));
		
		for y in 0 .. WINDOW_H {
			for x in 0 .. WINDOW_W {
				if self.gfx[(y as usize * WINDOW_W as usize) + x as usize] == 0 {
					img.put_pixel(x.into(), y.into(), Rgba([0, 0, 0, 255])); // disabled
				} else {
					img.put_pixel(x.into(), y.into(), Rgba([255, 255, 255, 255])); // enabled
				}
			}
		}

		self.draw_flag = false; // reset the draw flag
		img
	}

	fn play_snd() {
		
	}
}
