//! 练习 uniform1：uniform buffer 与 BindGroup
//!
//! 阶段：3（Buffer + MVP 相机）
//! 前置：mvp
//! 依赖：wgpu + glam
//!
//! 步骤
//! 1. 定义 `#[repr(C)]` 的 Uniform 结构体（MVP，注意 16 字节对齐）。
//! 2. 创建 uniform buffer（UNIFORM + COPY_DST）。
//! 3. 建 BindGroupLayout + BindGroup，绑定 buffer。
//! 4. 每帧 `queue.write_buffer` 更新 MVP。
//! 5. 加键盘/鼠标改相机位置或 FOV。
//!
//! 资料
//! - wgpu BindGroup: https://docs.rs/wgpu/latest/wgpu/struct.BindGroup.html
//! - learn-wgpu（uniforms）: https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/
//!
//! 验收
//! - uniform buffer + BindGroup
//! - queue.write_buffer 更新 uniform
//! - 加相机控制（位置 / FOV）
//!
//! 总结
//! 掌握 uniform 数据从 CPU 到 GPU 的路径：buffer → bind group → shader。

fn main() {
    // TODO: 创建 uniform buffer → BindGroup → write_buffer 更新 MVP。
    todo!("uniform buffer + BindGroup + write_buffer");
}
