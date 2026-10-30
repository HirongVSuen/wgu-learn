//! 练习 setup2：跑通空项目并确认 API
//!
//! 阶段：0（环境与版本）
//! 前置：setup1
//! 依赖：无（纯 Rust）
//!
//! 学习思路
//! 跑通空项目验证工具链；再对照文档确认 winit 0.30 用 `ApplicationHandler`、
//! wgpu 30 的 `request_device` 参数。遇到编译错误优先读错误提示 + docs.rs。
//!
//! 资料
//! - wgpu 文档: https://docs.rs/wgpu/
//! - winit 文档: https://docs.rs/winit/
//!
//! 验收
//! - 空项目 `cargo run` 跑通
//! - 确认 winit 0.30 使用 `ApplicationHandler`
//! - 确认 wgpu `request_device` 参数与锁定版本一致

fn main() {
    // TODO: 运行 `cargo run`；再对照 wgpu 30 的 request_device 签名。
    println!("阶段 0：跑通空项目");
}
