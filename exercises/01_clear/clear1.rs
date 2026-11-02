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
//!
//!

use wgpu_learn::{SimpleApp, event_loop_run_app};

fn main() {
    // TODO: 实现 ApplicationHandler，创建窗口并处理上述事件。
    let mut app = SimpleApp::default();
    event_loop_run_app(&mut app);
    // 实现完成后运行 `cargo run --bin clear1`，check() 会打印自查要点。
    check();
}

/// 自检：打印验证要点（视觉/行为反馈）。
fn check() {
    println!("自查清单（clear1）：");
    println!("  - 运行后能看到窗口");
    println!("  - 点击关闭（CloseRequested）程序正常退出");
    println!("  - 拖拽改变大小（Resized）不崩溃，宽高为 0 时不 configure");
}
