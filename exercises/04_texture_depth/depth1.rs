//! 练习 depth1：深度纹理与遮挡
//!
//! 阶段：4（纹理 + 深度）
//! 前置：texture1 + depth_linear
//! 依赖：wgpu
//!
//! 步骤
//! 1. 创建深度纹理（`Depth32Float`，usage 含 RENDER_ATTACHMENT）。
//! 2. pipeline 配 `DepthStencilState`（wgpu 30 用 Option 字段）。
//! 3. render pass 配深度附件 `depth_ops`。
//! 4. 画两个前后重叠的 3D 物体，观察遮挡。
//! 5. 把近平面调远/物体放远，观察 z-fighting。
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
//! 总结
//! 理解深度测试“开启/比较/写入”的配置，以及深度冲突的成因。
//!
//! 注意：wgpu 30 中 `DepthStencilState` 的 `depth_write_enabled` / `depth_compare`
//! 是 `Option` 类型；渲染 pass 的深度附件用 `depth_ops`。

fn main() {
    // TODO: 创建深度纹理 → DepthStencilState → 画多个 3D 物体。
    // 实现完成后运行 `cargo run --bin depth1`，check() 会打印自查要点。
    check();
}

/// 自检：打印验证要点（遮挡是否正确）。
fn check() {
    println!("自查清单（depth1）：");
    println!("  - 前面的物体正确遮挡后面的物体");
    println!("  - 深度比较函数（Less）与写入配置正确");
    println!("  - 把物体放远后能观察到深度冲突（z-fighting）");
}
