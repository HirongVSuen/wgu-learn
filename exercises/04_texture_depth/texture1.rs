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
    // 实现完成后运行 `cargo run --bin texture1`，check() 会打印自查要点。
    check();
}

/// 自检：打印验证要点（纹理是否正确显示）。
fn check() {
    println!("自查清单（texture1）：");
    println!("  - 物体表面显示纹理，颜色正确（注意 sRGB）");
    println!("  - 纹理坐标越界时按 sampler 的寻址模式处理");
    println!("  - 缩放纹理时过滤模式生效（线性/最近）");
}
