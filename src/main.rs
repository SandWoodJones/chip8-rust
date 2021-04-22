#[allow(non_snake_case)]
mod CPU;

use crate::CPU::CHIP8;

fn main() {
	let mut chip8 = CHIP8::new();
	chip8.load_program("demos/pong2.c8").unwrap();

	loop {
		chip8.emulate_cycle();
	}

}

