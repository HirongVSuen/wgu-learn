//! 练习 blend1：混合
//!
//! 阶段：6b（混合 / MSAA / 模板）
//! 前置：05_project
//! 依赖：wgpu
//!
//! 步骤
//! 1. 给 pipeline 的 color target 配 `BlendState::ALPHA_BLENDING`。
//! 2. 画一个半透明 quad。
//! 3. 理解 src_alpha + one_minus_src_alpha 的合成公式。
//! 4. 注意绘制顺序（先不透明后透明）。
//!
//! 资料
//! - wgpu BlendState: https://docs.rs/wgpu/latest/wgpu/struct.BlendState.html
//! - LearnOpenGL（混合）: https://learnopengl.com/Advanced-OpenGL/Blending
//!
//! 验收
//! - BlendState 配透明混合
//! - 理解 Alpha 合成、覆盖率、混合顺序
//!
//! 总结
//! 理解 Alpha 合成与混合顺序（先不透明、后透明、按距离排序）。

fn main() {
    // TODO: 配置 BlendState，画半透明物体观察混合。
    todo!("BlendState 透明混合");
}
