//! 练习 shadow1：阴影
//!
//! 阶段：6c（阴影）
//! 前置：05_project + 04 深度
//! 依赖：wgpu + glam
//!
//! 学习思路
//! 先从光源视角渲染深度生成 shadow map，再在主 pass 里比较采样深度。
//! 用 `SamplerCompare` 做硬件比较，调 bias 消除阴影瑕疵（acne）。
//!
//! 资料
//! - LearnOpenGL（Shadow Mapping）: https://learnopengl.com/Advanced-Lighting/Shadows/Shadow-Mapping
//! - wgpu Sampler: https://docs.rs/wgpu/latest/wgpu/struct.Sampler.html
//!
//! 验收
//! - 深度 pass 生成 shadow map
//! - 光照空间矩阵
//! - SamplerCompare
//! - shadow bias 调参

fn main() {
    // TODO: shadow map → SamplerCompare → bias。
    todo!("阴影映射");
}
