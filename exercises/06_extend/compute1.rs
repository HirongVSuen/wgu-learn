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
    todo!("GPU 计算");
}
