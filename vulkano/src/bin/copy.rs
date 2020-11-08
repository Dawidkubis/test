use vulkano::{
	device::{Device, DeviceExtensions, Features},
	instance::{Instance, InstanceExtensions, PhysicalDevice},
	buffer::{BufferUsage, CpuAccessibleBuffer},
	command_buffer::{
		AutoCommandBufferBuilder,
		CommandBuffer,
	},
	sync::GpuFuture
};

fn main() {
	// creating a Vulkan Instance.
	// this will fail if Vulkan is not supported on the system.
	let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();
	// choosing a physical device to execute commands on.
	// note: this iterator may be empty if none of the devices support vulkan.
	let physical = PhysicalDevice::enumerate(&instance).next().unwrap();

	// choosing a queue family.
	// something like threads on a cpu
	let queue_family = physical
		.queue_families()
		.find(|&q| q.supports_graphics())
		.expect("couldn't find a graphical queue family");

	// creating a device instance.
	// it is created from a physical device object
	// and with features that will be used
	// the last argument is an iterator of queue families
	// with a f32 corresponding to the priority.
	// note: there is no guarantee to the order
	// in which queue families will be used.
	// returns a device (Arc<Device>) and an iterator over queues.
	let (device, mut queues) = Device::new(
		physical,
		&Features::none(),
		&DeviceExtensions::none(),
		[(queue_family, 0.5)].iter().cloned(),
	)
	.unwrap();

	// i am choosing the queue since i know there is only one
	// (i chose one queue family)
	let queue = queues.next().unwrap();
	
	// creating a buffer and loading data into it.
	// they will be stored in the gpu.
	// i will be transmitting it from one buffer to another
	let source_content = 0 .. 64;
	let source = CpuAccessibleBuffer::from_iter(
		device.clone(), BufferUsage::all(), false, source_content,
	).unwrap();
	// the third bool argument asks if the buffer should be cpu cached
	// this will rarely give a performance boost, so should be false

	let dest_content = (0 .. 64).map(|_| 0);
	let dest = CpuAccessibleBuffer::from_iter(
		device.clone(), BufferUsage::all(), false, dest_content,
	).unwrap();

	// creating a command buffer.
	let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family())
		.unwrap();
	builder.copy_buffer(source.clone(), dest.clone()).unwrap();
	let command_buffer = builder.build().unwrap();

	// executing the commands
	let finished = command_buffer.execute(queue.clone()).unwrap();

	// waiting for the execution to finish
	finished.then_signal_fence_and_flush()
		.unwrap()
		.wait(None)
		.unwrap();

	// reading the data - works!
	println!("{:?}", &*dest.read().unwrap());
}
