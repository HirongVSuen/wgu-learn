//! 练习 depth1：深度纹理与遮挡
//!
//! 阶段：4（纹理 + 深度）
//! 前置：texture1 + depth_linear
//! 依赖：wgpu
//!
//! 学习思路
//! 深度纹理 + `DepthStencilState` 开启深度测试；画多个 3D 物体验证遮挡，
//! 并观察远处两个平面重叠时的深度冲突（z-fighting）。
//!
//! 资料
//! - wgpu DepthStencilState: https://docs.rs/wgpu/latest/wgpu/struct.DepthStencilState.html
//! - learn-wgpu（深度）: https://sotrh.github.io/learn-wgpu/beginner/tutorial8-depth/
//!
//! 验收
//! - 深度纹理 + DepthStencilState
//! - 画多个 3D 物体验证遮挡
//! - 能解释远处深度冲突
//!
//! 注意：wgpu 30 中 `DepthStencilState` 的 `depth_write_enabled` / `depth_compare`
//! 是 `Option` 类型；渲染 pass 的深度附件用 `depth_ops`。

fn main() {
    // TODO: 创建深度纹理 → DepthStencilState → 画多个 3D 物体。
    todo!("深度纹理 + DepthStencilState + 遮挡");
}
