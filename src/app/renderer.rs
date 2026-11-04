use std::{any, iter::once, sync::Arc};

use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, ExperimentalFeatures, Instance,
    InstanceDescriptor, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration,
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
        Self {
            instance: instance,
            adapter: adapter,
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            is_surface_configured: true,
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
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        }
        self.queue.submit(once(encoder.finish()));
        self.queue.present(output);
        Ok(())
    }
}
