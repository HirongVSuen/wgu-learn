//! 练习 clear1：winit 窗口与事件
//!
//! 阶段：1（窗口 + 清屏）
//! 前置：00_setup
//! 依赖：winit
//!
//! 学习思路
//! 先理解事件循环与 `ApplicationHandler`（`resumed` 建窗口、`window_event` 处理事件）。
//! 重点：`Resized` 宽高为 0 时不 configure，这是常见崩溃点。
//!
//! 资料
//! - winit 文档: https://docs.rs/winit/
//! - winit 仓库示例: https://github.com/rust-windowing/winit/tree/master/examples
//!
//! 验收
//! - 创建窗口
//! - 处理 CloseRequested / Resized / RedrawRequested

fn main() {
    // TODO: 实现 ApplicationHandler，创建窗口并处理上述事件。
    todo!("创建 EventLoop 与窗口");
}
