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
		event_loop::ControlFlow,
		window::WindowBuilder,
		dpi::LogicalSize
	},
	DrawConfig, Texture
};

use crate::graphics::GraphicalContext;

pub fn run(mut machine: CHIP8) {
	let window_bld = WindowBuilder::new()
			.with_inner_size(LogicalSize::new(WINDOW_W, WINDOW_H))
			.with_resizable(false);

	let gc = GraphicalContext::new(window_bld).unwrap();
	let GraphicalContext { ctx: mut context, .. } = gc; // take the context out of gc by destructuring it

	let mut screen_texture: Option<Texture> = None;

	gc.el.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
				WindowEvent::KeyboardInput { input, .. } => machine.handle_input(input.scancode),
				_ => ()
			},

			Event::MainEventsCleared => {
				let screen_image = machine.emulate_cycle();
				match screen_image {
					Some(s) => { 
						screen_texture = Some(Texture::from_image(&mut context, s).unwrap());
						context.window().request_redraw();
					},
					None => ()
				}
			},

			Event::RedrawRequested(..) => {
				match &screen_texture {
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
