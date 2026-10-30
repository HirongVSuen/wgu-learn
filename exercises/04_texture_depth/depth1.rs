//! 练习 depth1：深度纹理与遮挡
//!
//! 对应阶段 4：
//! - 深度纹理 + DepthStencilState
//! - 画多个 3D 物体验证遮挡
//!
//! 注意：wgpu 30 中 `DepthStencilState` 的 `depth_write_enabled` / `depth_compare`
//! 是 `Option` 类型；渲染 pass 的深度附件用 `depth_ops`。

fn main() {
    // TODO: 创建深度纹理 → DepthStencilState → 画多个 3D 物体。
    todo!("深度纹理 + DepthStencilState + 遮挡");
}
