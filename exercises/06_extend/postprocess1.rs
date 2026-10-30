//! 练习 postprocess1：多 Pass / 后处理
//!
//! 阶段：6a（多 Pass / 后处理）
//! 前置：05_project
//! 依赖：wgpu
//!
//! 步骤
//! 1. 创建离屏纹理（RENDER_ATTACHMENT + TEXTURE_BINDING）。
//! 2. 第一 pass 渲染场景到离屏纹理。
//! 3. 第二 pass 用全屏 quad 采样离屏纹理。
//! 4. 实现灰度（点积亮度）或模糊（3x3 卷积核）。
//! 5. 理解 LoadOp/StoreOp 在 pass 间的数据传递。
//!
//! 资料
//! - 图像卷积核: https://en.wikipedia.org/wiki/Kernel_(image_processing)
//! - learn-wgpu（后处理示例）: https://sotrh.github.io/learn-wgpu/
//!
//! 验收
//! - 离屏纹理 RENDER_ATTACHMENT
//! - 全屏 pass 采样上一 pass
//! - 理解 LoadOp/StoreOp
//! - 实现灰度或模糊（卷积、采样核、高斯权重、色调映射）
//!
//! 总结
//! 掌握离屏渲染 + 全屏后处理的套路，理解卷积/采样核。

fn main() {
    // TODO: 离屏纹理 → 全屏 pass → 灰度/模糊。
    todo!("多 pass 后处理");
}
