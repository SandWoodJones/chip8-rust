use crate::CPU::CHIP8;
use std::convert::TryInto;

use rand::random;

pub fn handle_opcode(opc: u16, cpu: &mut CHIP8) {
	match opc & 0xF000 { // To match the opcodes we only care about the first 4 bits
		0x0000 => { // There are multiple codes that start the first 4 bits as 0
			match opc & 0x000F { // Compare the last 4 bits
				0x0000 => { // 00E0 - Display - Clears the screen
					// ...
					cpu.pc += 2;
				},

				0x000E => { // 00EE - Flow - Returns from a subroutine
					cpu.sp -= 1; // decrease stack pointer
					cpu.pc = cpu.stack[cpu.sp as usize]; // puts the stored address into pc
					cpu.pc += 2;
				},

				_ => { // Illegal opcode
					println!("Unknown opcode [0x0000]: {:X}", opc);
				}
			}
		},

		0x1000 => { // 1NNN - Flow - Jumps to address NNN
			cpu.pc = opc & 0x0FFF;
		}

		0x2000 => { // 2NNN - Flow - Calls subroutine at NNN
			cpu.stack[cpu.sp as usize] = cpu.pc; // store the pc's current address in the stack
			cpu.sp += 1; // increase stack pointer
			cpu.pc = opc & 0x0FFF // gets rid of the first 4 bits and sets the pc to the address
		},

		0x3000 => { // 3XNN - Cond - Skip the next instruction if Vx == NN
			let vxi = ((opc & 0x0F00) >> 8) as usize;
			if cpu.V[vxi] as u16 == (opc & 0x00FF) {
				cpu.pc += 4;
			} else {
				cpu.pc += 2;
			}
		},

		0x4000 => { // 4XNN - Cond - Skip the next instruction if Vx != NN
			let vxi = ((opc & 0x0F00) >> 8) as usize;
			if cpu.V[vxi] as u16 != (opc & 0x00FF) {
				cpu.pc += 4;
			} else {
				cpu.pc += 2;
			}
		},

		0x5000 => { // 5XY0 - Cond - Skip the next instruction if Vx == Vy
			let vxi = ((opc & 0x0F00) >> 8) as usize;
			let vyi = ((opc & 0x00F0) >> 4) as usize;
			if cpu.V[vxi] == cpu.V[vyi] {
				cpu.pc += 4;
			} else {
				cpu.pc += 2;
			}
		}

		0x6000 => { // 6XNN - Const - Sets Vx to NN
			let vxi = ((opc & 0x0F00) >> 8) as usize;
			cpu.V[vxi] = (opc & 0x00FF) as u8;
			cpu.pc += 2;
		},

		0x7000 => { // 7XNN - Const - Adds NN to Vx
			let vxi = ((opc & 0x0F00) >> 8) as usize;
			let n: u8 = (opc & 0x00FF).try_into().unwrap();
			cpu.V[vxi] = cpu.V[vxi].wrapping_add(n);
			cpu.pc += 2;
		},

		0x8000 => {
			match opc & 0x000F {
				0x0000 => { // 8XY0 - Assign - Sets Vx to the value of Vy
					let vxi = ((opc & 0x0F00) >> 8) as usize; // get the index for V[x]
					let vyi = ((opc & 0x00F0) >> 4) as usize; // does the same for V[y]
					cpu.V[vxi] = cpu.V[vyi];
					cpu.pc += 2;
				},

				0x0001 => { // 8XY1 - BitOp - Sets Vx to Vx or Vy
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					let vyi = ((opc & 0x00F0) >> 4) as usize;
					cpu.V[vxi] = cpu.V[vxi] | cpu.V[vyi];
					cpu.pc += 2;
				},

				0x0002 => { // 8XY2 - BitOp - Sets Vx to Vx and Vy
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					let vyi = ((opc & 0x00F0) >> 4) as usize;
					cpu.V[vxi] = cpu.V[vxi] & cpu.V[vyi];
					cpu.pc += 2;
				},

				0x0003 => { // 8XY2 - BitOp - Sets Vx to Vx xor Vy
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					let vyi = ((opc & 0x00F0) >> 4) as usize;
					cpu.V[vxi] = cpu.V[vxi] ^ cpu.V[vyi];
					cpu.pc += 2;
				},

				0x0004 => { // 8XY4 - Math - Adds Vy to Vx. Vf is set to 1 when there's a carry, and to 0 when there's not
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					let vyi = ((opc & 0x00F0) >> 4) as usize;
					if cpu.V[vyi] > (255 - cpu.V[vxi]) { // there's overflow
						cpu.V[0xF] = 1; // set the carry bit
					} else {
						cpu.V[0xF] = 0;
					}
					cpu.V[vxi] = cpu.V[vxi].wrapping_add(cpu.V[vyi]);
					cpu.pc += 2;
				},

				0x0005 => { // 8XY5 - Math - Subtracts Vy from Vx. Vf is set to 0 when there's a borrow and to 1 when there's not
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					let vyi = ((opc & 0x00F0) >> 4) as usize;
					if cpu.V[vyi] > cpu.V[vxi] { // goes into negative
						cpu.V[0xF] = 0; // There's a borrow.
					} else {
						cpu.V[0xF] = 1;
					}
					cpu.V[vxi] = cpu.V[vxi].wrapping_sub(cpu.V[vyi]);
					cpu.pc += 2;
				},

				0x0006 => { // 8XY6 - BitOp - Stores the least significant bit of Vx in Vf and shifts Vx to the right by 1 
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					cpu.V[0xF] = cpu.V[vxi] & 0x1; // Get only the first bit
					cpu.V[vxi] >>= 1;
					cpu.pc += 2;
				}

				0x000E => { // 8XYE - BitOp - Stores the most significant bit of Vx in Vf and shifts Vx to the left by 1
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					cpu.V[0xF] = cpu.V[vxi] >> 7; // Shift the last bit into the first bit
					cpu.V[vxi] <<= 1;
					cpu.pc += 2;
				}

				_ => {
					println!("Unknown opcode [0x8000]: {:X}", opc);
				}
			}
		},

		0x9000 => { // 9XY0 - Cond - Skips the next instruction if Vx != Vy
			let vxi = ((opc & 0x0F00) >> 8) as usize;
			let vyi = ((opc & 0x00F0) >> 4) as usize;
			if cpu.V[vxi] != cpu.V[vyi] {
				cpu.pc += 4;
			} else {
				cpu.pc += 2;
			}
		}

		0xA000 => { // ANNN - MEM - Sets I to the address NNN
			cpu.I = opc & 0x0FFF; // gets rid of the first 4 bits and assigns the value to I
			cpu.pc += 2; // Since every instruction is 2 bytes long we increment the program counter by 2
		},

		0xB000 => { // BNNN - Flow - Jumps to the address NNN + V0
			cpu.pc = (opc & 0xFFF) + cpu.V[0] as u16;
		},

		0xC000 => { // CXNN - Rand - Sets Vx to the result of rnd(0, 255) and NN
			let vxi = ((opc & 0x0F00) >> 8) as usize;
			let n = random::<u8>();
			cpu.V[vxi] = n & (opc & 0x00FF) as u8;
			cpu.pc += 2;
		}

		0xD000 => { // DXYN - Disp -- Draw a sprite at (Vx, Vy) with a width of 8 and height of N+1
			// ...
			cpu.pc += 2;
		},

		0xE000 => {
			match opc & 0x00FF {
				0x009E => { // EX9E - KeyOp - Skips the next instruction if the key stored in Vx is pressed.
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					if cpu.key[cpu.V[vxi] as usize] != 0 {
						cpu.pc += 4;
					} else {
						cpu.pc += 2;
					}
				},
				0x00A1 => { // EXA1 - KeyOp - Skips the next instruction if the key stored in Vx isn't pressed.
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					if cpu.key[cpu.V[vxi] as usize] == 0 {
						cpu.pc += 4;
					} else {
						cpu.pc += 2;
					}
				},
				_ => {
					println!("Unknown opcode [0xE000]: {:X}", opc);
				}
			}
		},

		0xF000 => {
			match opc & 0x00FF {
				0x0007 => { // FX07 - Timer - Sets Vx to the value of the delay timer
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					cpu.V[vxi] = cpu.delay_timer;
					cpu.pc += 2;
				}

				0x0015 => { // FX15 - Timer - Sets the delay timer to Vx
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					cpu.delay_timer = cpu.V[vxi];
					cpu.pc += 2;
				},

				0x0018 => { // FX18 - Sound - Sets the sound timer to Vx
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					cpu.sound_timer = cpu.V[vxi];
					cpu.pc += 2;
				}

				0x0029 => { // FX29 - MEM - Sets I to the location of the sprite for the character in Vx.
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					cpu.I = (cpu.V[vxi] as u16) * 0x5;
					cpu.pc += 2;
				},

				0x0033 => { // FX33 - BCD - Stores the binary-coded-decimal representation of Vx at the address in I
					let vxi = ((opc & 0x0F00) >> 8) as usize;
					cpu.memory[cpu.I as usize] = cpu.V[vxi as usize] / 100;
					cpu.memory[(cpu.I + 1) as usize] = (cpu.V[vxi as usize] / 10) % 10;
					cpu.memory[(cpu.I + 2) as usize] = (cpu.V[vxi as usize] % 100) % 10;
					cpu.pc += 2;
				},

				0x0055 => { // FX55 - MEM - Stores V0 to Vx in memory starting at address I increasing the offset from I by 1 for each value. Then adds X+1 to I
					for i in 0 ..= (opc & 0x0F00) >> 8 { // go through V0 to Vx
						cpu.memory[(cpu.I + i) as usize] = cpu.V[i as usize];
					}
					cpu.I += 1 + (opc & 0x0F00) >> 8;
					cpu.pc += 2;
				},

				0x0065 => { // FX65 - MEM - Fills V0 to Vx with values from memory starting at address I increasing the offset from I by 1 for each value. Then adds X+1 to I
					for i in 0 ..= (opc & 0x0F00) >> 8 { // go through V0 to Vx
						cpu.V[i as usize] = cpu.memory[(cpu.I + i) as usize];
					}
					cpu.I += 1 + (opc & 0x0F00) >> 8;
					cpu.pc += 2;
				}

				_ => {
					println!("Unknown opcode [0xF000]: {:X}", opc);
				}
			}
		}

		_ => { // Illegal opcode
			println!("Unknown opcode: {:X}", opc);
		}
	}
	println!("opcode: {:X}", opc);
}
