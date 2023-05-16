use std::{borrow::Cow, iter};

use bytemuck::{Pod, Zeroable};
use leptos::log;
use web_sys::HtmlCanvasElement;
use wgpu::util::DeviceExt;

use crate::math_utils::{get_pixel_to_ndc_transform, get_transform};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    pos: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
struct ProjectionUniform {
    // Note that in principle we would like to use a `view_proj: [[f32; 3]; 3]` directly
    // corresponding to that mat3x3 in the shader here. However, there seems to be an
    // limitation of the alignment on the shader side: The shader always aligns vec3 at
    // 16 bytes, i.e., the same as a vec4. Apparently that also means that a mat3x3 must
    // actually by passed as 3 columns (or rows? semantics unclear) of vec4. See e.g.:
    // - https://sotrh.github.io/learn-wgpu/showcase/alignment/#alignment-of-vertex-and-index-buffers
    // - https://gist.github.com/teoxoy/936891c16c2a3d1c3c5e7204ac6cd76c
    // - https://stackoverflow.com/a/70849928/1804173
    // - https://github.com/hasenbanck/egui_wgpu_backend/pull/59/files
    // The error that results from getting the padding wrong is:
    // drawArraysInstanced: Buffer for uniform block is smaller than UNIFORM_BLOCK_DATA_SIZE.
    view_proj: [[f32; 4]; 3],
}

impl ProjectionUniform {
    fn new(w: u32, h: u32, dx: i32, dy: i32) -> Self {
        //let tx = get_pixel_to_ndc_transform(w, false);
        //let ty = get_pixel_to_ndc_transform(h, true);
        let tx = get_transform(0.0 + dx as f32, w as f32 + dx as f32, false);
        let ty = get_transform(0.0 + dy as f32, h as f32 + dy as f32, true);

        use cgmath::SquareMatrix;
        let mut m = cgmath::Matrix3::identity();
        m.x[0] = tx.m;
        m.y[1] = ty.m;
        m.z[0] = tx.c;
        m.z[1] = ty.c;
        Self {
            // Ideally, this should work, but currently not possible due to required padding.
            // view_proj: m.into(),
            view_proj: [
                [m.x[0], m.x[1], m.x[2], 0.0],
                [m.y[0], m.y[1], m.y[2], 0.0],
                [m.z[0], m.z[1], m.z[2], 0.0],
            ],
        }
    }
}

struct MsaaPipeline {
    bundle: wgpu::RenderBundle,
    multisampled_framebuffer: wgpu::TextureView,
    projection_buffer: wgpu::Buffer,
    sample_count: u32,
}

impl MsaaPipeline {
    pub fn new(
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        vertex_data: Vec<Vertex>,
    ) -> Self {
        let sample_count = {
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
            max_sample_count
        };
        log!("Using MSAA sample count: {}", sample_count);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let vertex_count = vertex_data.len() as u32;

        let (bundle, projection_buffer) =
            create_bundle(&device, &config, sample_count, &vertex_buffer, vertex_count);
        let multisampled_framebuffer =
            create_multisampled_framebuffer(&device, &config, sample_count);

        MsaaPipeline {
            bundle,
            multisampled_framebuffer,
            sample_count,
            projection_buffer,
        }
    }

    fn render(
        &self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        surface: &wgpu::Surface,
        queue: &wgpu::Queue,
    ) {
        let w = config.width;
        let h = config.height;
        let projection_uniform = ProjectionUniform::new(w, h, -20, -20);

        queue.write_buffer(
            &self.projection_buffer,
            0,
            bytemuck::cast_slice(&[projection_uniform]),
        );

        // The `surface_texture` is often called `frame` in the examples.
        let surface_texture = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        // Often just called `view` in the examples.
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let clear_color = wgpu::Color {
            r: 0.99,
            g: 0.99,
            b: 0.99,
            a: 1.0,
        };
        let rpass_color_attachment = if self.sample_count == 1 {
            wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: true,
                },
            }
        } else {
            wgpu::RenderPassColorAttachment {
                view: &self.multisampled_framebuffer,
                resolve_target: Some(&texture_view),
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
            .execute_bundles(iter::once(&self.bundle));

        queue.submit(iter::once(encoder.finish()));
        surface_texture.present();
    }
}

fn create_bundle(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    sample_count: u32,
    vertex_buffer: &wgpu::Buffer,
    vertex_count: u32,
) -> (wgpu::RenderBundle, wgpu::Buffer) {
    let w = config.width;
    let h = config.height;
    let projection_uniform = ProjectionUniform::new(w, h, 10, 10);

    let projection_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Projection Buffer"),
        contents: bytemuck::cast_slice(&[projection_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let projection_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("projection_bind_group_layout"),
        });

    let projection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &projection_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: projection_buffer.as_entire_binding(),
        }],
        label: Some("projection_bind_group"),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&projection_bind_group_layout],
        push_constant_ranges: &[],
    });

    let shader = &device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shaders/line.wgsl"))),
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
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
    encoder.set_bind_group(0, &projection_bind_group, &[]);
    encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
    encoder.draw(0..vertex_count, 0..1);
    let render_bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
        label: Some("main"),
    });

    (render_bundle, projection_buffer)
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

/// Equivalent of `winit::PhysicalSize<u32>` that gets returned by `window.inner_size()`.
#[derive(Clone, Copy, Debug)]
struct Size {
    width: u32,
    height: u32,
}

pub struct Renderer {
    // canvas: HtmlCanvasElement,
    adapter: wgpu::Adapter,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    msaa_pipeline: Option<MsaaPipeline>,
}

impl Renderer {
    pub async fn new(canvas: HtmlCanvasElement) -> Self {
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

        Self {
            adapter,
            surface,
            device,
            queue,
            config,
            msaa_pipeline: None,
        }
    }

    pub fn set_render_data(&mut self) {
        let mut vertex_data = vec![];

        /*
        let max = 50;
        for i in 0..max {
            let percent = i as f32 / max as f32;
            let (sin, cos) = (percent * 2.0 * std::f32::consts::PI).sin_cos();
            vertex_data.push(Vertex {
                pos: [0.0, 0.0],
                color: [1.0, -sin, cos, 1.0],
            });
            vertex_data.push(Vertex {
                pos: [1.0 * cos, 1.0 * sin],
                color: [sin, -cos, 1.0, 1.0],
            });
        }
        */
        vertex_data.push(Vertex {
            pos: [1.0, 1.0 - 0.5],
            color: [0.0, 0.0, 0.0, 1.0],
        });
        vertex_data.push(Vertex {
            pos: [1.0, 4.0 + 0.5],
            color: [0.0, 0.0, 0.0, 1.0],
        });
        vertex_data.push(Vertex {
            pos: [1.0 - 0.5, 6.0],
            color: [0.0, 0.0, 0.0, 1.0],
        });
        vertex_data.push(Vertex {
            pos: [4.0 + 0.5, 6.0],
            color: [0.0, 0.0, 0.0, 1.0],
        });

        let msaa_pipeline =
            MsaaPipeline::new(&self.adapter, &self.device, &self.config, vertex_data);
        self.msaa_pipeline = Some(msaa_pipeline);
    }

    pub fn render(&self) {
        log!("Re-rendering...");

        // This was taken from the Event::WindowEvent branch of the sample code.
        // Looks like it is crucial that this runs before any drawing happens.
        // TODO: This should be moved into a resize callback. For now call in each
        // rendering (even though we don't update size changes here anyway...)
        self.surface.configure(&self.device, &self.config);

        if let Some(msaa_pipeline) = self.msaa_pipeline.as_ref() {
            msaa_pipeline.render(&self.device, &self.config, &self.surface, &self.queue);
        }
    }
}
