use std::sync::Arc;
use vulkano::{
	device::{Device, DeviceExtensions, Features},
	instance::{Instance, InstanceExtensions, PhysicalDevice},
	buffer::{BufferUsage, CpuAccessibleBuffer},
	command_buffer::{
		AutoCommandBufferBuilder,
		CommandBuffer,
	},
	sync::GpuFuture,
	pipeline::ComputePipeline,
	descriptor::{
		descriptor_set::PersistentDescriptorSet,
		pipeline_layout::PipelineLayoutAbstract,
	},
	format::{
		Format,
		ClearValue,
	},
	image::{
		Dimensions,
		StorageImage,
	},
};

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

	// creating an image.
	// similar to creating a buffer, although not cpu accessible.
	// i pass dimensions, and format + iterator of queue families
	// that can access it
	let image = StorageImage::new(
		device.clone(), Dimensions::Dim2d { width: 1024, height: 1024 },
		Format::R8G8B8A8Unorm, Some(queue.family())
		).unwrap();
	
	// filling the image with color.
	// i need to ask the gpu to do it
	let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
		.unwrap()
		.clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0]))
		.unwrap()
		.build()
		.unwrap();

	// copying the data from the image to a buffer
	let buf = CpuAccessibleBuffer::from_iter(
		device.clone(), BufferUsage::all(), false, (0 .. 1024 * 1024 * 4).map(|_| 0u8)
		).unwrap();
}
