//! 练习 texture1：纹理与采样器
//!
//! 阶段：4（纹理 + 深度）
//! 前置：02_triangle（pipeline 基础）
//! 依赖：wgpu
//!
//! 学习思路
//! 纹理链：Texture → TextureView → Sampler → shader 采样。
//! 上传用 `queue.write_texture`；颜色纹理用 `*Srgb` 格式保证正确显示。
//!
//! 资料
//! - wgpu Texture: https://docs.rs/wgpu/latest/wgpu/struct.Texture.html
//! - learn-wgpu（纹理）: https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/
//! - sRGB: https://en.wikipedia.org/wiki/SRGB
//!
//! 验收
//! - 创建 Texture 与 TextureView
//! - 创建 Sampler
//! - 上传纹理数据，处理 sRGB
//! - shader 采样纹理

fn main() {
    // TODO: 创建纹理 → 上传数据 → sampler → shader 采样。
    todo!("Texture + TextureView + Sampler + sRGB");
}
