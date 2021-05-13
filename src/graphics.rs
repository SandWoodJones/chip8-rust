use crow::{
	glutin::{
		event_loop::EventLoop,
		window::WindowBuilder,
	},
	Context
};

use std::error::Error;

pub struct GraphicalContext {
	pub el: EventLoop<()>,
	pub ctx: Context
}

impl GraphicalContext {
	pub fn new(wb: WindowBuilder) -> Result<GraphicalContext, Box<dyn Error>> {
		let el = EventLoop::new();
		let ctx = Context::new(wb, &el)?;
		Ok( GraphicalContext { el, ctx } )
	}
}
