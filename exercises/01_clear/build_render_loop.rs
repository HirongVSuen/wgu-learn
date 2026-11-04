//! 练习 build_render_loop：渲染循环 + 错误处理（重构）
//!
//! 阶段：1（窗口 + 清屏）
//! 前置：clear3
//! 依赖：wgpu + winit
//!
//! 目标（学「造」）
//! 把 clear1~clear3 累积出来的「窗口 → 事件 → 清屏」串成一个稳定的渲染循环骨架：
//! 按需重绘、全程 Result 化、处理设备丢失/表面失效的恢复。对应引擎化路线「阶段 B」。
//!
//! 步骤
//! 1. 引入 `RedrawRequested` 驱动重绘，`ControlFlow` 按需（而非每帧空转）。
//! 2. `render()`：取帧 → encoder → pass(Clear) → submit → present，全 `Result` 化。
//! 3. 处理 `SurfaceError`：Lost（报错退出）/ Outdated（重新 configure）等。
//! 4. 把 clear1/clear2/clear3 里散落的逻辑收敛为稳定的 `update` / `render` / `resize` 接口。
//! 5. 用 `EngineError`（自定义）替代 `unwrap`，让失败可诊断。
//!
//! 资料
//! - wgpu SurfaceError: https://docs.rs/wgpu/latest/wgpu/enum.SurfaceError.html
//! - learn-wgpu（窗口 surface）: https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/
//!
//! 验收
//! - 渲染循环由 RedrawRequested 驱动，按需而非空转
//! - render / update / resize 接口稳定，全程 Result 化
//! - 设备丢失（Lost）与表面失效（Outdated）能正确恢复
//!
//! 总结
//! 理解「稳定帧循环」的价值：它是实体引擎与一次性 demo 的分水岭，
//! 也是引擎化路线阶段 B 的地基。
//!
//! 注意：wgpu 30 的 `get_current_texture` 返回 `CurrentSurfaceTexture` 枚举，
//! 各分支（Suboptimal / Outdated / Lost 等）都要显式处理。

use wgpu_learn::app::runner::run;

fn main() -> anyhow::Result<()> {
    // 已实现：渲染循环与错误处理见 src/app/（重构 clear1~3 的窗口/清屏逻辑）。
    run()?;
    // 实现完成后运行 `cargo run --bin build_render_loop`，check() 会打印自查要点。
    check();
    Ok(())
}

/// 自检：打印验证要点（帧循环 + 设备丢失恢复）。
fn check() {
    println!("自查清单（build_render_loop）：");
    println!("  - 渲染循环由 RedrawRequested 驱动，按需而非空转");
    println!("  - render / update / resize 接口稳定，全程 Result 化");
    println!("  - 设备丢失（Lost）与表面失效（Outdated）能正确恢复");
}
