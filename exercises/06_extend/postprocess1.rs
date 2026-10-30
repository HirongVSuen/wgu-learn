//! 练习 postprocess1：多 Pass / 后处理
//!
//! 阶段：6a（多 Pass / 后处理）
//! 前置：05_project
//! 依赖：wgpu
//!
//! 学习思路
//! 先渲染到离屏纹理（RENDER_ATTACHMENT），再用全屏 quad 采样上一 pass。
//! 理解 LoadOp/StoreOp 决定 pass 之间怎么传递数据；灰度/模糊本质是卷积核。
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

fn main() {
    // TODO: 离屏纹理 → 全屏 pass → 灰度/模糊。
    todo!("多 pass 后处理");
}
