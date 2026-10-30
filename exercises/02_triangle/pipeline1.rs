//! 练习 pipeline1：RenderPipeline 与 draw
//!
//! 阶段：2（三角形 + shader）
//! 前置：shader1
//! 依赖：wgpu
//!
//! 步骤
//! 1. 用 ShaderModule 配 `VertexState` 与 `FragmentState`（wgpu 30 的 entry_point 是 Option，需 compilation_options）。
//! 2. 配置 `primitive`（TriangleList）、`multisample`、color target。
//! 3. `create_render_pipeline`。
//! 4. 在 render pass 里 `set_pipeline` + `draw(0..3)`。
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
//! 总结
//! 理解 pipeline 固定管线的组成，以及 shader 如何被组装成可执行的管线。
//!
//! 注意：wgpu 30 中 `VertexState`/`FragmentState` 的 `entry_point` 是
//! `Option<&str>`，且需提供 `compilation_options`。

fn main() {
    // TODO: 创建 ShaderModule → RenderPipeline → draw 三角形。
    // 实现完成后运行 `cargo run --bin pipeline1`，check() 会打印自查要点。
    check();
}

/// 自检：打印验证要点（三角形是否出现）。
fn check() {
    println!("自查清单（pipeline1）：");
    println!("  - 画面中能看到一个三角形");
    println!("  - 改 fragment 颜色能生效");
    println!("  - 不 panic，窗口可正常关闭");
}
