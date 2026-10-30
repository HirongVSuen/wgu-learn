//! 练习 setup2：跑通空项目并确认 API
//!
//! 阶段：0（环境与版本）
//! 前置：setup1
//! 依赖：无（纯 Rust）
//!
//! 步骤
//! 1. `cargo run` 跑通空项目（默认 bin 打印索引）。
//! 2. 对照 docs.rs 确认 winit 0.30 的事件模型是 `ApplicationHandler` trait。
//! 3. 对照 wgpu 30 文档确认 `request_device` 的参数（label / features / limits / memory_hints / trace）。
//! 4. 记下这两个会随版本变化的关键 API，避免后续踩坑。
//!
//! 资料
//! - wgpu 文档: https://docs.rs/wgpu/
//! - winit 文档: https://docs.rs/winit/
//!
//! 验收
//! - 空项目 cargo run 跑通
//! - 确认 winit 0.30 使用 ApplicationHandler
//! - 确认 wgpu request_device 参数与锁定版本一致
//!
//! 总结
//! 确认工具链可用，并为两个易变 API 建立版本对应关系。

fn main() {
    // TODO: 运行 `cargo run`；再对照 wgpu 30 的 request_device 签名。
    check();
}

/// 自检：打印核对要点（阶段 0）。
fn check() {
    println!("自查清单（阶段 0）：");
    println!("  - 空项目 `cargo run` 跑通");
    println!("  - 确认 winit 0.30 使用 ApplicationHandler");
    println!("  - 确认 wgpu 30 request_device 参数与锁定版本一致");
}
