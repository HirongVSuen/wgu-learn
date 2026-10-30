//! 练习 texture1：纹理与采样器
//!
//! 阶段：4（纹理 + 深度）
//! 前置：02_triangle（pipeline 基础）
//! 依赖：wgpu
//!
//! 步骤
//! 1. 创建 Texture（`Rgba8UnormSrgb` 或 `*Srgb`），usage 含 TEXTURE_BINDING 与 COPY_DST。
//! 2. `queue.write_texture` 上传像素。
//! 3. 创建 Sampler（线性过滤 + repeat）。
//! 4. shader 里 `textureSample` 采样，输出到片元。
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
//!
//! 总结
//! 掌握纹理上传/采样链路，理解 sRGB 格式对颜色正确性的意义。

fn main() {
    // TODO: 创建纹理 → 上传数据 → sampler → shader 采样。
    todo!("Texture + TextureView + Sampler + sRGB");
}
