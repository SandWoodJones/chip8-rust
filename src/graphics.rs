use crow::{
	glutin::{
		event_loop::EventLoop,
		window::WindowBuilder,
	},
	Context, Texture
};

use std::error::Error;

pub struct GraphicalContext {
	pub el: EventLoop<()>,
	pub ctx: Context,
	pub txt: Option<Texture>
}

impl GraphicalContext {
	pub fn new(wb: WindowBuilder) -> Result<GraphicalContext, Box<dyn Error>> {
		let el = EventLoop::new();
		let ctx = Context::new(wb, &el)?;
		let txt = None;

		Ok( GraphicalContext { el, ctx, txt } )
	}
}
