#[allow(non_snake_case)]
pub mod CPU;

mod graphics;

static FONTSET: [u8; 80] = [ 0xF0, 0x90, 0x90, 0x90, 0xF0,	 // 0
							 0x20, 0x60, 0x20, 0x20, 0x70,	 // 1
							 0xF0, 0x10, 0xF0, 0x80, 0xF0,	 // 2
							 0xF0, 0x10, 0xF0, 0x10, 0xF0,	 // 3
							 0x90, 0x90, 0xF0, 0x10, 0x10,	 // 4
							 0xF0, 0x80, 0xF0, 0x10, 0xF0,	 // 5
							 0xF0, 0x80, 0xF0, 0x90, 0xF0,	 // 6
							 0xF0, 0x10, 0x20, 0x40, 0x40,	 // 7
							 0xF0, 0x90, 0xF0, 0x90, 0xF0,	 // 8
							 0xF0, 0x90, 0xF0, 0x10, 0xF0,	 // 9
							 0xF0, 0x90, 0xF0, 0x90, 0x90,	 // A
							 0xE0, 0x90, 0xE0, 0x90, 0xE0,	 // B
							 0xF0, 0x80, 0x80, 0x80, 0xF0,	 // C
							 0xE0, 0x90, 0x90, 0x90, 0xE0,	 // D
							 0xF0, 0x80, 0xF0, 0x80, 0xF0,	 // E
							 0xF0, 0x80, 0xF0, 0x80, 0x80 ]; // F

const WINDOW_W: u8 = 64;
const WINDOW_H: u8 = 32;
const VIRTUAL_WW: u8 = 192;
const VIRTUAL_WH: u8 = 96;

// define the pieces of the cpu
#[allow(non_snake_case)]
pub struct CHIP8 {
	opcode: u16, // current opcode
	memory: [u8; 4096], // ram
	V: [u8; 16], // cpu registers
	I: u16, // index register
	pc: u16, // program counter
	gfx: [u8; WINDOW_W as usize * WINDOW_H as usize], // visual ram
	draw_flag: bool, // since the cpu doesn't draw each frame we set a flag for when it should
	sound_flag: bool, // the same as above but for sound
	delay_timer: u8,
	sound_timer: u8,
	stack: [u16; 16],
	sp: u16, // stack pointer
	key: [u8; 16]
}

use crow::{
	glutin::{
		event::{ Event, WindowEvent, VirtualKeyCode },
		event_loop::ControlFlow,
		window::WindowBuilder,
		dpi::LogicalSize
	},
	DrawConfig, Texture
};

use crate::graphics::GraphicalContext;

use rodio::{ OutputStream, Sink };

use std::{
	time,
	path::Path
};

pub fn run(mut machine: CHIP8, program_path: &String) {
	// load sound data
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&stream_handle).unwrap();

	let beep_sound = load_sound_file("beep.ogg").unwrap();

	let program_name = Path::new(&program_path).file_name().unwrap()
						.to_str().unwrap();

	let window_bld = WindowBuilder::new()
			.with_title(format!("CHIP-8 {}", program_name))
			.with_inner_size(LogicalSize::new(VIRTUAL_WW, VIRTUAL_WH))
			.with_resizable(false);

	// create a graphical context and take the texture and context out of it through destructuring. TODO: this won't be required with rust version 2021
	let gc = GraphicalContext::new(window_bld).unwrap();
	let GraphicalContext { ctx: mut context, txt: mut screen_texture, .. } = gc;
	
	// create a draw config for setting window scale
	let drw_cfg = DrawConfig {
		scale: (3, 3),
		.. Default::default()
	};

	gc.el.run(move |event, _, control_flow| {
		let next_frame_time = time::Instant::now() + time::Duration::from_micros(1);
		*control_flow = ControlFlow::WaitUntil(next_frame_time);

		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
				WindowEvent::KeyboardInput { input, .. } => {
					match input.virtual_keycode {
						Some(kc) => match kc {
							VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
							_ => ()
						},
						_ => ()
					}
					machine.handle_input(input)
				},

				_ => ()
			},

			Event::MainEventsCleared => {
				machine.emulate_cycle();
				if machine.draw_flag {
					let screen_image = machine.create_screen_image();
					screen_texture = Some(Texture::from_image(&mut context, screen_image).unwrap());
					context.window().request_redraw();
				}
				if machine.sound_flag {
					sink.append(beep_sound.clone());
					machine.sound_flag = false;
				}
			},

			Event::RedrawRequested(..) => {
				match &screen_texture {
					Some(txt) => {
						let mut surface = context.surface();

						context.draw(&mut surface, &txt, (0, 0), &drw_cfg);

						context.present(surface).unwrap(); // swap back-buffer
					},
					None => ()
				}

			},

			_ => ()
		}
	});
}

use rodio::{
	Decoder, Source,
	source::Buffered
};
use std::{
	fs::File,
	io::{ self, Read },
	error::Error
};

// Read a file in binary mode and store it in a vector buffer
fn load_binary_file(path: &str) -> io::Result<Vec<u8>> {
	let mut p = File::open(path)?;
	let mut buf = Vec::new();

	p.read_to_end(&mut buf)?;
	Ok(buf)
}

// Loads a sound file as a Buffered struct. Thanks to https://stackoverflow.com/a/61547339/9353072
fn load_sound_file(path: &str) -> Result<Buffered<Decoder<File>> , Box<dyn Error>> {
	// load a sound from a file
	let file = File::open(path)?;

	// decode that file into a source
	let source = Decoder::new(file)?;

	// store the decoded audio in a buffer
	let result = source.buffered();

	// as the buffer is lazily initiated we force the decoding early
	result.clone().for_each(|_| {});

	Ok(result)
}
