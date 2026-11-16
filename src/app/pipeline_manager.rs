use std::collections::HashMap;

use wgpu::{
    ColorTargetState, Device, FragmentState, MultisampleState, PipelineLayoutDescriptor,
    PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    SurfaceConfiguration, VertexState,
};

// 管线管理
pub struct PipelineManager {
    index: HashMap<PipeLineType, RenderPipeline>,
}

impl PipelineManager {
    // 创建空的管理器。
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    // 命中缓存直接返回，未命中则创建并缓存后返回。
    pub fn get_or_create(
        &mut self,
        pipe_line_type: PipeLineType,
        device: &Device,
        config: &SurfaceConfiguration,
    ) -> &RenderPipeline {
        if !self.index.contains_key(&pipe_line_type) {
            let pipeline = Self::create_pipe_line(device, config);
            self.index.insert(pipe_line_type, pipeline);
        }
        self.index
            .get(&pipe_line_type)
            .expect("刚插入，缓存中必定存在该管线")
    }

    // 释放指定 key 的管线，下次 get_or_create 会重新创建。
    pub fn release(&mut self, pipe_line_type: PipeLineType) {
        self.index.remove(&pipe_line_type);
    }

    // 创建管线
    fn create_pipe_line(device: &Device, config: &SurfaceConfiguration) -> RenderPipeline {
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

        render_pipeline
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum PipeLineType {
    Solid,
    Wireframe,
    Highlight,
}
