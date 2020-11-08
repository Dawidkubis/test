// a structure describing a vertex in 2d space
#[derive(Default, Copy, Clone)]
struct Vertex {
	position: [f32; 2],
}

fn main() {
	// SETUP
	let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();
	let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
	let queue_family = physical
		.queue_families()
		.find(|&q| q.supports_graphics())
		.expect("couldn't find a graphical queue family");
	let (device, mut queues) = Device::new(
		physical,
		&Features::none(),
		&DeviceExtensions::none(),
		[(queue_family, 0.5)].iter().cloned(),
	)
	.unwrap();
	let queue = queues.next().unwrap();
	// SETUP END

	// register vertex
	vulkano::impl_vertex!(Vertex, position);
}
