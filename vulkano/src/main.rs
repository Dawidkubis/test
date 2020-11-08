use std::sync::Arc;
use vulkano::{
	device::{Device, DeviceExtensions, Features},
	instance::{Instance, InstanceExtensions, PhysicalDevice},
	buffer::{BufferUsage, CpuAccessibleBuffer},
	command_buffer::{
		AutoCommandBufferBuilder,
		CommandBuffer,
		DynamicState,
	},
	framebuffer::{Framebuffer, Subpass},
	sync::GpuFuture,
	format::Format,
	pipeline::{ComputePipeline, viewport::Viewport, GraphicsPipeline},
	image::{StorageImage, Dimensions},
};
use image::{Rgba, ImageBuffer};

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

	// create 3 vertices
	let vertex1 = Vertex { position: [-0.5, -0.5] };
	let vertex2 = Vertex { position: [0.0, 0.5] };
	let vertex3 = Vertex { position: [0.5, -0.25] };

	// put them in a buffer
	let vertex_buffer = CpuAccessibleBuffer::from_iter(
		device.clone(),
		BufferUsage::all(),
		false,
		vec![vertex1, vertex2, vertex3].into_iter(),
		).unwrap();

	// create image
	// vulkan will draw to
    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d {
            width: 1024,
            height: 1024,
        },
        Format::R8G8B8A8Unorm,
        Some(queue.family()),
    )
    .unwrap();

	// create buffer to read the image
    let buf = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("failed to create buffer");

	
	// Vertex shader, (type vertex)
	// it runs for every vertex in vertex_buffer
	// and positions it in space
	// gl_Position describes properties of the vertex and the first
	// two arguments are the position
	mod vs {
		vulkano_shaders::shader!{
			ty: "vertex",
			src: "
	#version 450

	layout(location = 0) in vec2 position;

	void main() {
		gl_Position = vec4(position, 0.0, 1.0);
	}"
		}
	}

	// fragment shader
	// runs for every pixel inside the triangle
	// this will fill the triangle red
	mod fs {
		vulkano_shaders::shader!{
			ty: "fragment",
			src: "
	#version 450

	layout(location = 0) out vec4 f_color;

	void main() {
		f_color = vec4(1.0, 0.0, 0.0, 1.0);
	}"
		}
	}

	// load both shaders
	let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
	let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

	// create a render pass
	// basically like n word pass but for rendering
	// comes with a bunch of settings
    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: Format::R8G8B8A8Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap(),
    );

	// the framebuffer contains the image we draw on
    let framebuffer = Arc::new(
        Framebuffer::start(render_pass.clone())
            .add(image.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

	// create the graphics pipeline needed for rendering
    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );
	
	// explains the image
    let dynamic_state = DynamicState {
        viewports: Some(vec![Viewport {
            origin: [0.0, 0.0],
            dimensions: [1024.0, 1024.0],
            depth_range: 0.0..1.0,
        }]),
        ..DynamicState::none()
    };

	// creating command builder
	let mut builder = AutoCommandBufferBuilder::
	primary_one_time_submit(device.clone(), queue.family()).unwrap();

	builder
		.begin_render_pass(
            framebuffer.clone(),
            false,
            vec![[0.0, 0.0, 1.0, 1.0].into()],
        )
        .unwrap()
        .draw(
            pipeline.clone(),
            &dynamic_state,
            vertex_buffer.clone(),
            (),
            (),
        )
        .unwrap()
        .end_render_pass()
        .unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone())
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("render.png").unwrap();
}
