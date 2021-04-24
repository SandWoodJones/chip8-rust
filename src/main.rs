use chip8::CHIP8;

use std::env;
use std::process;

fn main() {
	let args: Vec<_> = env::args().collect();

	let machine = CHIP8::new(&args)
						.unwrap_or_else(
							|e| {
								eprintln!("Error creating emulator object: {}", e);
								process::exit(1);
							}
						);

	chip8::run(machine);
}

