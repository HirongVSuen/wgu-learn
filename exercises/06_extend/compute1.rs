//! 练习 compute1：Compute
//!
//! 阶段：6d（Compute）
//! 前置：03_camera（buffer / bind group 基础）
//! 依赖：wgpu
//!
//! 步骤
//! 1. 写 compute shader（`@compute @workgroup_size`）。
//! 2. 创建 storage buffer（STORAGE）。
//! 3. 创建 ComputePipeline + BindGroup。
//! 4. `dispatch_workgroups` 执行，结果留在 GPU。
//!
//! 资料
//! - wgpu ComputePass: https://docs.rs/wgpu/latest/wgpu/struct.ComputePass.html
//! - learn-wgpu（compute 示例）: https://sotrh.github.io/learn-wgpu/showcase/compute/
//!
//! 验收
//! - ComputePipeline
//! - storage buffer
//! - dispatch_workgroups
//! - GPU 粒子/模拟，CPU 不回读
//!
//! 总结
//! 掌握 compute pipeline 与 storage buffer 的使用，理解 workgroup 划分与并行归约。

fn main() {
    // TODO: ComputePipeline + storage buffer + dispatch。
    // 实现完成后运行 `cargo run --bin compute1`，check() 会打印自查要点。
    check();
}

/// 自检：打印验证要点（计算是否在 GPU 上完成）。
fn check() {
    println!("自查清单（compute1）：");
    println!("  - compute shader 正确 dispatch（workgroup 划分合理）");
    println!("  - 结果写在 storage buffer，且 GPU 上可见");
    println!("  - 尽量不回读，整条链路留在 GPU");
}
