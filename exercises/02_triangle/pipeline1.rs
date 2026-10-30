//! 练习 pipeline1：RenderPipeline 与 draw
//!
//! 阶段：2（三角形 + shader）
//! 前置：shader1
//! 依赖：wgpu
//!
//! 学习思路
//! 流程：ShaderModule → RenderPipeline → `RenderPass::draw`。
//! 理解 pipeline 固定管线各阶段（vertex/primitive/multisample/fragment targets）。
//!
//! 资料
//! - wgpu RenderPipeline: https://docs.rs/wgpu/latest/wgpu/struct.RenderPipeline.html
//! - learn-wgpu（pipeline 章节）: https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/
//!
//! 验收
//! - 创建 ShaderModule
//! - 创建 RenderPipeline（先最小可用）
//! - RenderPass::draw 画三角形
//!
//! 注意：wgpu 30 中 `VertexState`/`FragmentState` 的 `entry_point` 是
//! `Option<&str>`，且需提供 `compilation_options`。

fn main() {
    // TODO: 创建 ShaderModule → RenderPipeline → draw 三角形。
    todo!("ShaderModule → RenderPipeline → draw");
}
