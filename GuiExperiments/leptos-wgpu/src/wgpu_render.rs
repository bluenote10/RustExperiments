use std::{borrow::Cow, iter};

use bytemuck::{Pod, Zeroable};
use leptos::log;
use web_sys::HtmlCanvasElement;
use wgpu::util::DeviceExt;

#[allow(dead_code)]
pub async fn render_triangle(canvas: &HtmlCanvasElement) {
    let width = canvas.width();
    let height = canvas.height();

    let instance = wgpu::Instance::default();

    let surface = instance
        .create_surface_from_canvas(&canvas)
        .expect("Failed to get surface");

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/minimal.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    let frame = surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        rpass.set_pipeline(&render_pipeline);
        rpass.draw(0..3, 0..1);
    }

    queue.submit(Some(encoder.finish()));
    frame.present();
    log!("Rendered successfully");
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    _pos: [f32; 2],
    _color: [f32; 4],
}

/// Equivalent of `winit::PhysicalSize<u32>` that gets returned by `window.inner_size()`.
#[derive(Clone, Copy, Debug)]
struct Size {
    width: u32,
    height: u32,
}

pub async fn render_msaa_line(canvas: &HtmlCanvasElement) {
    let size = Size {
        width: canvas.width(),
        height: canvas.height(),
    };

    let instance = wgpu::Instance::default();

    let surface = instance
        .create_surface_from_canvas(&canvas)
        .expect("Failed to get surface");
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let config = surface
        .get_default_config(&adapter, size.width, size.height)
        .expect("Surface isn't supported by the adapter.");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty()
                    | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let sample_flags = adapter.get_texture_format_features(config.format).flags;

    let max_sample_count = {
        if sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X8) {
            8
        } else if sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X4) {
            4
        } else if sample_flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X2) {
            2
        } else {
            1
        }
    };
    let sample_count = max_sample_count;
    log!("Using MSAA sample count: {}", sample_count);

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/line.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let mut vertex_data = vec![];

    let max = 50;
    for i in 0..max {
        let percent = i as f32 / max as f32;
        let (sin, cos) = (percent * 2.0 * std::f32::consts::PI).sin_cos();
        vertex_data.push(Vertex {
            _pos: [0.0, 0.0],
            _color: [1.0, -sin, cos, 1.0],
        });
        vertex_data.push(Vertex {
            _pos: [1.0 * cos, 1.0 * sin],
            _color: [sin, -cos, 1.0, 1.0],
        });
    }

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertex_data),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let vertex_count = vertex_data.len() as u32;

    let bundle = create_bundle(
        &device,
        &config,
        &shader,
        &pipeline_layout,
        sample_count,
        &vertex_buffer,
        vertex_count,
    );
    let multisampled_framebuffer = create_multisampled_framebuffer(&device, &config, sample_count);

    // This was taken from the Event::WindowEvent branch of the sample code.
    // Looks like it is crucial that this runs before any drawing happens.
    surface.configure(&device, &config);

    // This was taken from the Event::RedrawRequested branch of the sample code.
    log!("Getting surface texture...");
    let frame = surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    log!("Getting surface texture [successful]");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let clear_color = wgpu::Color {
            r: 0.99,
            g: 0.99,
            b: 0.99,
            a: 1.0,
        };
        let rpass_color_attachment = if sample_count == 1 {
            wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: true,
                },
            }
        } else {
            wgpu::RenderPassColorAttachment {
                view: &multisampled_framebuffer,
                resolve_target: Some(&view),
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    // Storing pre-resolve MSAA data is unnecessary if it isn't used later.
                    // On tile-based GPU, avoid store can reduce your app's memory footprint.
                    store: false,
                },
            }
        };

        encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(rpass_color_attachment)],
                depth_stencil_attachment: None,
            })
            .execute_bundles(iter::once(&bundle));
    }

    queue.submit(iter::once(encoder.finish()));
    frame.present();
}

fn create_bundle(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
    pipeline_layout: &wgpu::PipelineLayout,
    sample_count: u32,
    vertex_buffer: &wgpu::Buffer,
    vertex_count: u32,
) -> wgpu::RenderBundle {
    log!("sample_count: {}", sample_count);
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets: &[Some(config.format.into())],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            front_face: wgpu::FrontFace::Ccw,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: sample_count,
            ..Default::default()
        },
        multiview: None,
    });
    let mut encoder = device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
        label: None,
        color_formats: &[Some(config.format)],
        depth_stencil: None,
        sample_count,
        multiview: None,
    });
    encoder.set_pipeline(&pipeline);
    encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
    encoder.draw(0..vertex_count, 0..1);
    encoder.finish(&wgpu::RenderBundleDescriptor {
        label: Some("main"),
    })
}

fn create_multisampled_framebuffer(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    sample_count: u32,
) -> wgpu::TextureView {
    let multisampled_texture_extent = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };
    let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
        size: multisampled_texture_extent,
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: config.format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        label: None,
        view_formats: &[],
    };

    device
        .create_texture(multisampled_frame_descriptor)
        .create_view(&wgpu::TextureViewDescriptor::default())
}

pub struct Renderer {
    canvas: HtmlCanvasElement,
}

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        Self { canvas }
    }

    pub async fn render(&self) {
        log!("Re-rendering...");
        render_msaa_line(&self.canvas).await;
    }
}
