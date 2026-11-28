use wgpu::util::DeviceExt;

const TRIANGLE_VERTICES: [[f32; 3]; 3] = [
    [0.0, 0.6, 0.0],
    [-0.6, -0.6, 0.0],
    [0.6, -0.6, 0.0],
];

const IDENTITY_TRANSFORM: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub(crate) struct Renderer {
    surface: wgpu::Surface<'static>, //Surface connects the renderer to the window so we can present frames.
    device: wgpu::Device, //evice is our connection to the GPU. We’ll use it to create resources such as textures and to record rendering commands.
    queue: wgpu::Queue, //queue submits those recorded commands to the GPU for execution
    config: wgpu::SurfaceConfiguration, //config stores the surface settings, including its width, height, and pixel format. Keeping it in Renderer lets us update the dimensions when the window resizes.
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,    
}

impl Renderer {
    pub(crate) async fn new(window: std::sync::Arc<winit::window::Window>) -> Self { //uses async so it can wait for GPU setup,
        let instance = wgpu::Instance::default(); //Creates the entry point to wgpu with default settings.
        let size = window.inner_size(); //) reads the width and height of the window’s drawable area in physical pixels. We’ll use those dimensions to configure the surface.
        let surface = instance.create_surface(window).expect("Failed to create surface"); //Creates the surface that will present frames in the window.
        let adapter_options = wgpu::RequestAdapterOptions { //Opens the options for selecting a graphics adapter. Next we’ll tell wgpu to pick one that can present to this surface.
                compatible_surface: Some(&surface), //Requires an adapter that can present frames to this surface. &surface borrows it for adapter selection.
                ..Default::default()
            };
        let adapter = instance.request_adapter(&adapter_options).await.expect("No compatible GPU"); //Requests an adapter that can present to our surface. .await waits for the result, and .expect(...) stops initialization with a clear message if the request fails.
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default())
            .await.expect("Failed to create GPU device");
        let config = surface.get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .expect("No supported surface configuration");
        surface.configure(&device, &config); //Applies the configuration so the surface can provide textures for rendering and presentation.
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle vertex buffer"),
            contents: bytemuck::cast_slice(&TRIANGLE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform uniform buffer"),
        })
        let shader = device.create_shader_module(wgpu::include_wgsl!("triangle.wgsl"));
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { //Starts defining the render pipeline, which tells the GPU which shaders to run and how to write their output.
            label: Some("Triangle pipeline"), //Gives the pipeline a name that can appear in validation errors and graphics debuggers.
            layout: None, //Lets wgpu infer the pipeline’s resource layout from the shaders. The triangle shader needs no bound resources.
            vertex: wgpu::VertexState { //Opens the vertex stage configuration. Next we’ll tell it to run vs_main from the shader module
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                })],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            multiview_mask: None,
            cache: None,
        });
        Self { surface, device, queue, config, pipeline, vertex_buffer } //Builds the Renderer using the four local variables as fields. This is the method’s final expression, so Rust returns it.
    } 

    pub(crate) fn render(&self) { //A crate is a unit of Rust code that the compiler builds into an executable or a library.
        let frame = match self.surface.get_current_texture() { //Asks the surface for the texture we’ll draw into and starts matching on the acquisition result.
            wgpu::CurrentSurfaceTexture::Success(frame) => frame, //On success, extracts the acquired texture and binds it to frame
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame, //Suboptimal still supplies a usable frame, even though the surface settings are out of date.
            _ => return, //_ catches the remaining outcomes. For this first render pass, return skips the draw whenever acquisition supplies no texture.
        };
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                store: wgpu::StoreOp::Store,
            }
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(color_attachment)],
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..3, 0..1);
        drop(pass);
        self.queue.submit([encoder.finish()]); //encoder.finish() turns the recorded commands into a command buffer.
        self.queue.present(frame);
    }
    pub(crate) fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return; 
        }
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }
}