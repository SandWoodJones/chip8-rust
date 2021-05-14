use crate::{ CPU::CHIP8, WINDOW_W, WINDOW_H };
use std::convert::TryInto;

use rand::random;

#[cfg(test)]
mod tests;

impl CHIP8 {
	pub fn handle_opcode(&mut self) {
		let vxi = ((self.opcode & 0x0F00) >> 8) as usize; // turn X (2nd digit of an opcode) into an index
		let vyi = ((self.opcode & 0x00F0) >> 4) as usize; // same but for Y (3rd digit of an opcode)

		match self.opcode & 0xF000 { // To match the opcodes we only care about the first 4 bits
			0x0000 => { // There are multiple codes that start the first 4 bits as 0
				match self.opcode & 0x000F { // Compare the last 4 bits
					0x0000 => { // 00E0 - Display - Clears the screen
						for i in 0 .. 2048 {
							self.gfx[i] = 0x00 // a black pixel is represented as 0
						}
						self.draw_flag = true;
						self.pc += 2;
					},

					0x000E => { // 00EE - Flow - Returns from a subroutine
						self.sp -= 1; // decrease stack pointer
						self.pc = self.stack[self.sp as usize]; // puts the stored address into pc
						self.pc += 2;
					},

					_ => { // Illegal opcode
						println!("Unknown opcode [0x0000]: {:X}", self.opcode);
					}
				}
			},

			0x1000 => { // 1NNN - Flow - Jumps to address NNN
				self.pc = self.opcode & 0x0FFF;
			},

			0x2000 => { // 2NNN - Flow - Calls subroutine at NNN
				self.stack[self.sp as usize] = self.pc; // store the pc's current address in the stack
				self.sp += 1; // increase stack pointer
				self.pc = self.opcode & 0x0FFF // gets rid of the first 4 bits and sets the pc to the address
			},

			0x3000 => { // 3XNN - Cond - Skip the next instruction if Vx == NN
				if self.V[vxi] as u16 == (self.opcode & 0x00FF) {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			0x4000 => { // 4XNN - Cond - Skip the next instruction if Vx != NN
				if self.V[vxi] as u16 != (self.opcode & 0x00FF) {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			},

			0x5000 => { // 5XY0 - Cond - Skip the next instruction if Vx == Vy
				if self.V[vxi] == self.V[vyi] {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			}

			0x6000 => { // 6XNN - Const - Sets Vx to NN
				self.V[vxi] = (self.opcode & 0x00FF) as u8;
				self.pc += 2;
			},

			0x7000 => { // 7XNN - Const - Adds NN to Vx
				let n: u8 = (self.opcode & 0x00FF).try_into().unwrap();
				self.V[vxi] = self.V[vxi].wrapping_add(n);
				self.pc += 2;
			},

			0x8000 => {
				match self.opcode & 0x000F {
					0x0000 => { // 8XY0 - Assign - Sets Vx to the value of Vy
						self.V[vxi] = self.V[vyi];
						self.pc += 2;
					},

					0x0001 => { // 8XY1 - BitOp - Sets Vx to Vx or Vy
						self.V[vxi] = self.V[vxi] | self.V[vyi];
						self.pc += 2;
					},

					0x0002 => { // 8XY2 - BitOp - Sets Vx to Vx and Vy
						self.V[vxi] = self.V[vxi] & self.V[vyi];
						self.pc += 2;
					},

					0x0003 => { // 8XY2 - BitOp - Sets Vx to Vx xor Vy
						self.V[vxi] = self.V[vxi] ^ self.V[vyi];
						self.pc += 2;
					},

					0x0004 => { // 8XY4 - Math - Adds Vy to Vx. sets Vf to 1 if theres a carry and 0 if there isnt
						if self.V[vyi] > (255 - self.V[vxi]) { // there's overflow
							self.V[0xF] = 1; // set the carry bit
						} else {
							self.V[0xF] = 0;
						}

						self.V[vxi] = self.V[vxi].wrapping_add(self.V[vyi]);
						self.pc += 2;
					},

					0x0005 => { // 8XY5 - Math - Subs Vy from Vx. sets Vf to 0 if theres a borrow and 1 if there isnt
						if self.V[vyi] > self.V[vxi] { // goes into negative
							self.V[0xF] = 1; // There's a borrow.
						} else {
							self.V[0xF] = 0;
						}
						self.V[vxi] = self.V[vxi].wrapping_sub(self.V[vyi]);
						self.pc += 2;
					},

					0x0006 => { // 8XY6 - BitOp - Stores the lsb of Vx in Vf. Shifts Vx to the right by 1
						self.V[0xF] = self.V[vxi] & 0x1; // Get only the least significant bit
						self.V[vxi] >>= 1; // shift by 1
						self.pc += 2;
					},

					0x000E => { // 8XYE - BitOp - Stores the msb of Vx in Vf. Shifts Vx to the left by 1
						self.V[0xF] = self.V[vxi] >> 7; // Get only the most significant bit
						self.V[vxi] <<= 1;
						self.pc += 2;
					},

					_ => {
						println!("Unknown opcode [0x8000]: {:X}", self.opcode);
					}
				}
			},

			0x9000 => { // 9XY0 - Cond - Skips the next instruction if Vx != Vy
				if self.V[vxi] != self.V[vyi] {
					self.pc += 4;
				} else {
					self.pc += 2;
				}
			}

			0xA000 => { // ANNN - MEM - Sets I to the address NNN
				self.I = self.opcode & 0x0FFF; // gets rid of the first 4 bits and assigns the value to I
				self.pc += 2; // Since every instruction is 2 bytes long we increment the program counter by 2
			},

			0xB000 => { // BNNN - Flow - Jumps to the address NNN + V0
				self.pc = (self.opcode & 0xFFF) + self.V[0] as u16;
			},

			0xC000 => { // CXNN - Rand - Sets Vx to the result of rnd(0, 255) and NN
				let n = random::<u8>(); // will generate a number between 0 and 255 (ranges of an u8)
				self.V[vxi] = n & (self.opcode & 0x00FF) as u8;
				self.pc += 2;
			}

			0xD000 => { // DXYN - Disp -- Draw a sprite at (Vx,Vy) of width 8 and height N+1. Vf is set to 1 if any pixels get flipped and to 0 if not.
				let height = (self.opcode & 0x000F) as usize;
				self.V[0xF] = 0; // reset the Vf register

				for yline in 0 .. height { // for each row
					let sprite = self.memory[self.I as usize + yline]; // get the pixel data stored at I + yline

					for xline in 0 .. 8 { // for each bit of a row
						let cur_pixel = sprite & (0x80 >> xline); // scans through the sprite byte, 1 bit at a time.
						if cur_pixel != 0 { // if the current pixel is set to 1
							// access the screen's 2D array of pixels with 1D indexing.
							let screen_coords = 
								(vxi + xline + ((vyi + yline) * 64)) % (WINDOW_W as usize * WINDOW_H as usize);

							// check if the pixel on display at that position is already set to 1, If it
							// is set the Vf register accordingly.
							if self.gfx[screen_coords] == 1 { self.V[0xF] = 1; }

							// finally, update the screen's pixels with the new values using XOR
							self.gfx[screen_coords] ^= 1;
						}
					}
				}

				self.draw_flag = true; // the vram was changed so we must redraw
				self.pc += 2;
			},

			0xE000 => {
				match self.opcode & 0x00FF {
					0x009E => { // EX9E - KeyOp - Skips the next instruction if the key stored in Vx is pressed.
						if self.key[self.V[vxi] as usize] != 0 {
							self.pc += 4;
						} else {
							self.pc += 2;
						}
					},
					0x00A1 => { // EXA1 - KeyOp - Skips the next instruction if the key stored in Vx isn't pressed.
						if self.key[self.V[vxi] as usize] == 0 {
							self.pc += 4;
						} else {
							self.pc += 2;
						}
					},
					_ => {
						println!("Unknown opcode [0xE000]: {:X}", self.opcode);
					}
				}
			},

			0xF000 => {
				match self.opcode & 0x00FF {
					0x0007 => { // FX07 - Timer - Sets Vx to the value of the delay timer
						self.V[vxi] = self.delay_timer;
						self.pc += 2;
					}

					0x0015 => { // FX15 - Timer - Sets the delay timer to Vx
						self.delay_timer = self.V[vxi];
						self.pc += 2;
					},

					0x0018 => { // FX18 - Sound - Sets the sound timer to Vx
						self.sound_timer = self.V[vxi];
						self.pc += 2;
					},

					0x001E => { // FX1E - MEM - Adds Vx to I. Vf not affected
						self.I += self.V[vxi] as u16;
						self.pc += 2;
					}

					0x0029 => { // FX29 - MEM - Sets I to the location of the sprite for the character in Vx.
						self.I = (self.V[vxi] as u16) * 0x5;
						self.pc += 2;
					},

					0x0033 => { // FX33 - BCD - Stores the decimal representation of Vx at the address in I
						self.memory[self.I as usize] = self.V[vxi as usize] / 100;
						self.memory[(self.I + 1) as usize] = (self.V[vxi as usize] / 10) % 10;
						self.memory[(self.I + 2) as usize] = (self.V[vxi as usize] % 100) % 10;
						self.pc += 2;
					},

					0x0055 => { // FX55 - MEM - Stores V0 to Vx in memory, starts at I, adds X+1 to I
						for i in 0 ..= (self.opcode & 0x0F00) >> 8 { // go through V0 to Vx
							self.memory[(self.I + i) as usize] = self.V[i as usize];
						}
						self.I += 1 + (self.opcode & 0x0F00) >> 8;
						self.pc += 2;
					},

					0x0065 => { // FX65 - MEM - Fills V0 to Vx with values from memory, starts at I, adds X+1 to I
						for i in 0 ..= (self.opcode & 0x0F00) >> 8 { // go through V0 to Vx
							self.V[i as usize] = self.memory[(self.I + i) as usize];
						}
						self.I += 1 + (self.opcode & 0x0F00) >> 8;
						self.pc += 2;
					},

					_ => {
						println!("Unknown opcode [0xF000]: {:X}", self.opcode);
					}
				}
			},

			_ => { // Illegal opcode
				println!("Unknown opcode: {:X}", self.opcode);
			}
		}
	}
}
