//! 练习 uniform1：uniform buffer 与 BindGroup
//!
//! 阶段：3（Buffer + MVP 相机）
//! 前置：mvp
//! 依赖：wgpu + glam
//!
//! 学习思路
//! 把 MVP 放进 uniform buffer，用 BindGroup 绑到 shader；每帧 `write_buffer` 更新。
//! 注意 uniform 结构体 16 字节对齐，可用 `#[repr(C)]` + bytemuck 派生。
//!
//! 资料
//! - wgpu BindGroup: https://docs.rs/wgpu/latest/wgpu/struct.BindGroup.html
//! - learn-wgpu（uniforms）: https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/
//!
//! 验收
//! - uniform buffer + BindGroup
//! - queue.write_buffer 更新 uniform
//! - 加相机控制（位置 / FOV）

fn main() {
    // TODO: 创建 uniform buffer → BindGroup → write_buffer 更新 MVP。
    todo!("uniform buffer + BindGroup + write_buffer");
}
