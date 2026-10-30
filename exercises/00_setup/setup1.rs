//! 练习 setup1：创建 Cargo 项目并锁定依赖版本
//!
//! 阶段：0（环境与版本）
//! 前置：无（起点）
//! 依赖：无（纯 Rust，无需编译 wgpu）
//!
//! 步骤
//! 1. 阅读 README 路线图，建立阶段 0→7 的整体预期。
//! 2. 在 Cargo.toml 用 `=版本` 精确锁定 wgpu / winit / glam / pollster / env_logger。
//! 3. 把这些版本号写进 README 的版本表。
//! 4. 运行 `cargo run`，确认能编译运行。
//!
//! 资料
//! - Rust 官方学习资源: https://www.rust-lang.org/learn
//! - Cargo 依赖声明: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
//!
//! 验收
//! - Cargo.toml 锁定版本
//! - 版本号写入 README
//! - cargo run 能编译运行
//!
//! 总结
//! 建立可复现的环境基线；理解“锁定版本 + 记录版本”对后续排错的价值。

fn main() {
    // TODO: 核对 Cargo.toml 与 README 的版本是否一致。
    check();
}

/// 自检：打印核对要点（阶段 0）。
fn check() {
    println!("自查清单（阶段 0）：");
    println!("  - Cargo.toml 已用 `=版本` 锁定 wgpu/winint/glam/pollster/env_logger");
    println!("  - README 版本表与 Cargo.toml 一致");
    println!("  - `cargo run` 能编译运行");
}
