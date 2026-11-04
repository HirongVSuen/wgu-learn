//! 练习 clear3：清屏与 present
//!
//! 阶段：1（窗口 + 清屏）
//! 前置：clear2
//! 依赖：wgpu + env_logger + log
//!
//! 步骤
//! 1. `get_current_texture` 取帧（wgpu 30 返回 `CurrentSurfaceTexture` 枚举）。
//! 2. `create_view` 得到颜色附件视图。
//! 3. `create_command_encoder` + `begin_render_pass`，用 `LoadOp::Clear` 清屏。
//! 4. `queue.submit` + `queue.present`。
//! 5. 打印 adapter 名称/backend，接入 env_logger。
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
//! 总结
//! 掌握“取帧 → 编码 → 提交 → 呈现”四步，并会用日志确认后端。
//!
//! 注意：wgpu 30 中 `get_current_texture` 返回 `CurrentSurfaceTexture` 枚举，
//! 用 `Queue::present` 而不是 `frame.present()`。

use wgpu_learn::app::runner::run;

fn main() -> anyhow::Result<()> {
    // 已实现：清屏与 present 见 src/app/renderer.rs（Renderer::render）。
    run()?;
    // 实现完成后运行 `cargo run --bin clear3`，check() 会打印自查要点。
    check();
    Ok(())
}

/// 自检：打印验证要点（清屏颜色 + 日志）。
fn check() {
    println!("自查清单（clear3）：");
    println!("  - 窗口显示你设置的清屏背景色");
    println!("  - 日志输出 adapter 名称与 backend（RUST_LOG=info）");
    println!("  - resize 后清屏正常，不崩溃");
}
