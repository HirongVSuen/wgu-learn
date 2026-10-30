//! 练习 clear3：清屏与 present
//!
//! 阶段：1（窗口 + 清屏）
//! 前置：clear2
//! 依赖：wgpu + env_logger + log
//!
//! 学习思路
//! 渲染三步：取当前帧 → 编码 render pass（Clear）→ submit + present。
//! 理解 LoadOp::Clear 的初值；打印 adapter 名称/backend 帮助确认后端。
//!
//! 资料
//! - wgpu RenderPass: https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html
//! - learn-wgpu: https://sotrh.github.io/learn-wgpu/
//!
//! 验收
//! - get_current_texture → RenderPass → submit → present
//! - 打印 adapter 名称与 backend
//! - 接入 env_logger
//!
//! 注意：wgpu 30 中 `get_current_texture` 返回 `CurrentSurfaceTexture` 枚举，
//! 用 `Queue::present` 而不是 `frame.present()`。

fn main() {
    // TODO: 写清屏 pass、提交、present；打印 adapter/backend 日志。
    todo!("get_current_texture → RenderPass → submit → present");
}
