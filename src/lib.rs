#[allow(non_snake_case)]
pub mod CPU;

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
	delay_timer: u8,
	sound_timer: u8,
	stack: [u16; 16],
	sp: u16, // stack pointer
	key: [u8; 16]
}

use crow::{
	glutin::{
		event::{ Event, WindowEvent },
		event_loop::{ EventLoop, ControlFlow },
		window::WindowBuilder,
		dpi::LogicalSize
	},
	Context, DrawConfig, Texture
};

pub fn run(mut machine: CHIP8) {
	let event_loop = EventLoop::new();
	let window_bld = WindowBuilder::new()
			.with_inner_size(LogicalSize::new(WINDOW_W, WINDOW_H))
			.with_resizable(false);

	let mut context = Context::new(window_bld, &event_loop).unwrap();
	
	let mut texture: Option<Texture> = None;

	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
				_ => ()
			},

			Event::MainEventsCleared => {
				let screen = machine.emulate_cycle();
				match screen {
					Some(s) => { 
						texture = Some(Texture::from_image(&mut context, s).unwrap());
						context.window().request_redraw();
					},
					None => ()
				}
			},

			Event::RedrawRequested(..) => {
				match &texture {
					Some(t) => {
						let mut surface = context.surface();

						context.draw(&mut surface, &t, (0, 0), &DrawConfig::default());

						context.present(surface).unwrap(); // swap back-buffer
					},
					None => ()
				}

			},

			_ => ()
		}
	});
}
