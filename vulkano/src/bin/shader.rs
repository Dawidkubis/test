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
};

fn main() {
	// SETUP
	let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();
	let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
	let queue_family = physical
		.queue_families()
		.find(|&q| q.supports_graphics())
		.expect("couldn't find a graphical queue family");
	// here i add a device extension.
	// otherwise panic
	let (device, mut queues) = Device::new(
		physical,
		&Features::none(),
		&DeviceExtensions{
			khr_storage_buffer_storage_class: true,
			..DeviceExtensions::none()
		},
		[(queue_family, 0.5)].iter().cloned(),
	)
	.unwrap();
	let queue = queues.next().unwrap();

	// actual thing.
	// here is a data buffer containing 65536 numbers
	let data_iter = 0 .. 65536;
	let data_buffer = CpuAccessibleBuffer::from_iter(
		device.clone(), BufferUsage::all(), false, data_iter,
		).unwrap();

	// here is a shader that is to be compiled.
	// it will multiply anything that is in the set 0 binding 0 by 12 -
	// we are gonna put our data_buffer in there.
	mod cs {
    	vulkano_shaders::shader!{
        	ty: "compute",
        	src: "
#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
	uint data[];
} buf;

void main() {
	uint idx = gl_GlobalInvocationID.x;
	buf.data[idx] *= 12;
}
"
    	}
	}

	// here i am loading the shader.
	let shader = cs::Shader::load(device.clone()).unwrap();

	// creating a compute pipeline, with the shader's main entry point.
	let compute_pipeline = Arc::new(
		ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
		.unwrap()
	);

	// no idea what i am doing here
	// i think im describing where the data should be pointing
	let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
	// and now here is my descriptor set with the buffer inside
	let set = Arc::new(
		PersistentDescriptorSet::start(layout.clone())
			.add_buffer(data_buffer.clone()).unwrap()
			.build()
			.unwrap()
	);

	// command buffer ready
	let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family())
		.unwrap();
	builder.dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ())
		.unwrap();

	let command_buffer = builder
		.build()
		.unwrap();

	// execution and waiting
	let finished = command_buffer.execute(queue.clone()).unwrap();
	finished.then_signal_fence_and_flush().unwrap()
		.wait(None)
		.unwrap();

	// and here are my values
	// much faster than with cpu
	println!("{:?}", &*data_buffer.read().unwrap());
}
