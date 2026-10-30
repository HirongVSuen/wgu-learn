//! 练习 clear3：清屏与 present
//!
//! 对应阶段 1：
//! - get_current_texture → RenderPass → submit → present
//! - 打印 adapter 名称和 backend
//! - 接入 env_logger
//!
//! 注意：wgpu 30 中 `get_current_texture` 返回 `CurrentSurfaceTexture` 枚举，
//! 用 `Queue::present` 而不是 `frame.present()`。

fn main() {
    // TODO: 写清屏 pass、提交、present；打印 adapter/backend 日志。
    todo!("get_current_texture → RenderPass → submit → present");
}
