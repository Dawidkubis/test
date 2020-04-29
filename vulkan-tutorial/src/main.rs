use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use vulkano::instance::Instance;

struct Triangle {
	instance: Arc<Instance>,
	window: Window,
	event_loop: EventLoop<()>,
}

impl Triangle {
	pub fn new() -> Self {
		let event_loop = EventLoop::new();
		let window = Window::new(&event_loop).unwrap();

		let instance = Self::create_instance();
		
		Self {
			instance,
			window,
			event_loop,
		}
	}

	fn create_instance() -> Arc<Instance> {
		let required_extensions = vulkano_win::required_extensions();
		Instance::new(None, &required_extensions, None)
			.unwrap()
	}

	fn run(self) {
		self.event_loop.run(move |event, _, control_flow| {
			*control_flow = ControlFlow::Wait;

			match event {
				Event::WindowEvent {
					event: WindowEvent::CloseRequested,
					..
				} => *control_flow = ControlFlow::Exit,

				_ => (),
			}

		});
	}
}

fn main() {
	let app = Triangle::new();
	app.run();
}
