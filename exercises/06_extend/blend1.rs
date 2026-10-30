//! 练习 blend1：混合
//!
//! 阶段：6b（混合 / MSAA / 模板）
//! 前置：05_project
//! 依赖：wgpu
//!
//! 学习思路
//! `BlendState::ALPHA_BLENDING` 做非预乘 Alpha 混合；
//! 理解 Alpha 合成公式与混合顺序（先画不透明、再画透明、按距离排序）。
//!
//! 资料
//! - wgpu BlendState: https://docs.rs/wgpu/latest/wgpu/struct.BlendState.html
//! - LearnOpenGL（混合）: https://learnopengl.com/Advanced-OpenGL/Blending
//!
//! 验收
//! - BlendState 配透明混合
//! - 理解 Alpha 合成、覆盖率、混合顺序

fn main() {
    // TODO: 配置 BlendState，画半透明物体观察混合。
    todo!("BlendState 透明混合");
}
