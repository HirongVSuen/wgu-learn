//! 练习 clear1：winit 窗口与事件
//!
//! 阶段：1（窗口 + 清屏）
//! 前置：00_setup
//! 依赖：winit
//!
//! 步骤
//! 1. `EventLoop::new()` 创建事件循环，`set_control_flow`。
//! 2. 实现 `ApplicationHandler`：在 `resumed` 里 `create_window`。
//! 3. 在 `window_event` 处理 `CloseRequested`（exit）、`Resized`（宽高 0 跳过）、`RedrawRequested`。
//! 4. 用 `run_app` 启动循环。
//!
//! 资料
//! - winit 文档: https://docs.rs/winit/
//! - winit 仓库示例: https://github.com/rust-windowing/winit/tree/master/examples
//!
//! 验收
//! - 创建窗口
//! - 处理 CloseRequested / Resized / RedrawRequested
//!
//! 总结
//! 理解事件驱动模型：`resumed` 建窗口、`window_event` 处理事件；resize 为 0 不 configure 是关键防御。

fn main() {
    // TODO: 实现 ApplicationHandler，创建窗口并处理上述事件。
    todo!("创建 EventLoop 与窗口");
}
