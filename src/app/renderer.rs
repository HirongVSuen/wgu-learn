use std::{any, iter::once, sync::Arc};

use wgpu::{
    Adapter, Backends, ColorTargetState, Device, DeviceDescriptor, ExperimentalFeatures,
    FragmentState, Instance, InstanceDescriptor, MultisampleState, PipelineLayoutDescriptor,
    PrimitiveState, Queue, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    ShaderModuleDescriptor, Surface, SurfaceConfiguration, VertexState,
    rwh::{HasDisplayHandle, HasWindowHandle},
};

use crate::core::action::Action;

pub struct Renderer {
    instance: Instance,
    adapter: Adapter,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: RenderPipeline,
}

// 用给定的窗口句柄创建一个新的 Renderer 实例。
impl Renderer {
    // 用给定的窗口句柄初始化一个新的 Renderer 实例。
    pub async fn new<W: HasDisplayHandle + HasWindowHandle + Send + Sync + 'static>(
        window: Arc<W>,
        width: u32,
        height: u32,
    ) -> Self {
        // 创建一个新的 WGPU 实例。
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..InstanceDescriptor::new_without_display_handle()
        });
        // 创建一个新的 WGPU 画布。
        let surface = instance.create_surface(window.clone()).unwrap();

        // 请求一个显卡适配器。
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true,
            })
            .await
            .unwrap();
        // 创建一个新的 WGPU 设备和队列。
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: ExperimentalFeatures::disabled(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();
        // 获取画布的支持格式。
        let surface_caps = surface.get_capabilities(&adapter);
        // 选择一个支持的格式。
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .unwrap_or(&surface_caps.formats[0]);
        // 配置文件
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *surface_format,
            width: width,
            height: height,
            present_mode: wgpu::PresentMode::Fifo,
            color_space: wgpu::SurfaceColorSpace::Auto,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        // 真正配置画布，并打印 adapter 名称与 backend。
        surface.configure(&device, &config);

        log::info!(
            "adapter: {} ({:?})",
            adapter.get_info().name,
            adapter.get_info().backend
        );

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../wgsl/shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("pipe_line layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            cache: None,
            multiview_mask: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        Self {
            instance: instance,
            adapter: adapter,
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            is_surface_configured: true,
            render_pipeline: render_pipeline,
        }
    }

    // 调整窗口大小时调用，更新配置并重置 surface 配置状态。
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.is_surface_configured = true;
        }
    }

    // 处理键盘输入事件。
    pub fn handle_keyboard_input(&mut self, action: Action) {
        match action {
            Action::None => {}
        }
    }

    // 更新渲染状态。
    pub fn update(&mut self) {}

    // 渲染一帧。
    pub fn render(&mut self) -> anyhow::Result<()> {
        if !self.is_surface_configured {
            return Ok(());
        }
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Surface texture lost");
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render code"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(once(encoder.finish()));
        self.queue.present(output);
        Ok(())
    }
}
