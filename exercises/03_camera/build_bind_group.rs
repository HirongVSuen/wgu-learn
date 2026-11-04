//! 练习 bind_group_assets：BindGroup 资源化（重构）
//!
//! 阶段：3（Buffer + MVP 相机）
//! 前置：uniform1
//! 依赖：wgpu + glam
//!
//! 目标（学「造」）
//! 把 uniform1 里散在 renderer 的 uniform buffer + BindGroup 创建逻辑，
//! 重构为可复用的资源管理：uniform 数据与 bind group 统一持有、按需更新，
//! 避免每帧手写 write_buffer 与重复建 bind group。对应引擎化路线「阶段 C」。
//!
//! 步骤
//! 1. 定义 `#[repr(C)]` 的 Uniform 结构体（MVP，16 字节对齐）。
//! 2. 实现 uniform buffer 的创建与持久持有（UNIFORM + COPY_DST）。
//! 3. 建 BindGroupLayout + BindGroup，封装为独立的资源类型。
//! 4. 提供 `write_buffer` 的封装方法，统一更新入口。
//! 5. 重构 renderer：不再在 `new()`/`update()` 里散写这些创建代码。
//!
//! 资料
//! - wgpu BindGroup: https://docs.rs/wgpu/latest/wgpu/struct.BindGroup.html
//! - learn-wgpu（uniforms）: https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/
//!
//! 验收
//! - uniform buffer 与 bind group 由独立类型持有，不再散在 renderer
//! - 更新 uniform 有统一入口（而非裸 write_buffer）
//! - 16 字节对齐，无校验错误
//!
//! 总结
//! 理解「资源化」：把高频创建、反复更新的 GPU 资源（buffer/bind group）封装成
//! 有明确生命周期的类型，是资源管理器（阶段 C）的重要组成。
//!
//! 注意：wgpu 30 的 BindGroupLayout 需要 `entries` + `immediate_size`。

use wgpu_learn::app::runner::run;

fn main() -> anyhow::Result<()> {
    // 已实现：BindGroup 资源化见 src/app/（重构 uniform1 的创建逻辑）。
    run()?;
    // 实现完成后运行 `cargo run --bin build_bind_group`，check() 会打印自查要点。
    check();
    Ok(())
}

/// 自检：打印验证要点（资源封装 + 更新入口）。
fn check() {
    println!("自查清单（build_bind_group）：");
    println!("  - uniform buffer 与 bind group 由独立类型持有");
    println!("  - 更新 uniform 有统一封装入口，不裸写 write_buffer");
    println!("  - 结构体 16 字节对齐，无校验错误");
}
