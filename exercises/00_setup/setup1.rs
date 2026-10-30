//! 练习 setup1：创建 Cargo 项目并锁定依赖版本
//!
//! 阶段：0（环境与版本）
//! 前置：无（起点）
//! 依赖：无（纯 Rust，无需编译 wgpu）
//!
//! 学习思路
//! 先整体浏览 README 里阶段 0→7 的路线图，建立“每步学什么”的预期；
//! 锁定版本是为了可复现，把版本写进 README 是记录环境基线。
//!
//! 资料
//! - Rust 官方学习资源: https://www.rust-lang.org/learn
//! - Cargo 依赖声明: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
//!
//! 验收
//! - `Cargo.toml` 锁定 wgpu / winit / glam / pollster / env_logger 版本
//! - 版本号写入 README
//! - `cargo run` 能编译运行

fn main() {
    // TODO: 检查 Cargo.toml 依赖版本与 README 是否一致。
    println!("阶段 0：环境与版本");
}
