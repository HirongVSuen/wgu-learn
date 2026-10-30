//! 练习 compute1：Compute
//!
//! 阶段：6d（Compute）
//! 前置：03_camera（buffer / bind group 基础）
//! 依赖：wgpu
//!
//! 学习思路
//! ComputePipeline + storage buffer（STORAGE 用途）+ `dispatch_workgroups`。
//! 理解 workgroup 划分与并行归约；做 GPU 模拟时尽量不回读（保持全在 GPU 侧）。
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

fn main() {
    // TODO: ComputePipeline + storage buffer + dispatch。
    todo!("GPU 计算");
}
