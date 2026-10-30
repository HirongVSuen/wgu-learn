//! 练习 uniform1：uniform buffer 与 BindGroup
//!
//! 对应阶段 3：
//! - uniform buffer + BindGroup
//! - `queue.write_buffer` 更新 uniform
//! - 加相机控制（位置 / FOV）
//!
//! 提示：uniform 结构体需要 `#[repr(C)]` 并对齐（16 字节），可用 bytemuck 派生。

fn main() {
    // TODO: 创建 uniform buffer → BindGroup → write_buffer 更新 MVP。
    todo!("uniform buffer + BindGroup + write_buffer");
}
