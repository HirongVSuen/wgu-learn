//! 练习 pipeline_manager：PipelineManager 管线缓存库（重构）
//!
//! 阶段：2（三角形 + shader）
//! 前置：pipeline1
//! 依赖：wgpu
//!
//! 目标（学「造」）
//! 把 pipeline1 里散在 renderer 中的 pipeline 创建逻辑，重构为可复用的
//! PipelineManager：按 key 缓存、复用管线，避免重复编译 shader / 重复建 pipeline。
//! 对应引擎化路线「方案一」与「阶段 C」。
//!
//! 步骤
//! 1. 定义 pipeline 的唯一标识（如枚举 `PipelineId` 或字符串 key）。
//! 2. 实现 `PipelineManager`：持有 `HashMap<Key, RenderPipeline>` 的缓存。
//! 3. 提供 `get_or_create(key, device, desc)`：命中缓存直接返回，未命中则创建并缓存。
//! 4. 重构 renderer：不再在 `new()` 里直接建 pipeline，改由 manager 统一管理。
//! 5. 提供 `release(key)` 释放不再使用的管线。
//!
//! 资料
//! - wgpu RenderPipeline: https://docs.rs/wgpu/latest/wgpu/struct.RenderPipeline.html
//! - 资源缓存/句柄模式: https://en.wikipedia.org/wiki/Reference_counting
//!
//! 验收
//! - 能缓存并复用 pipeline（相同 key 第二次返回同一实例）
//! - 释放后能重建
//! - renderer 不再持有 pipeline 创建的散代码
//!
//! 总结
//! 理解「管线缓存库」的价值：把高频创建的高成本资源（pipeline）集中管理、按需复用，
//! 是引擎资源管理（阶段 C）的第一个抓手。
//!
//! 注意：wgpu 30 的 `create_render_pipeline` 依赖 `ShaderModule` + `PipelineLayout`，
//! 这些也应纳入 manager 的管理，避免每次重建。

use wgpu_learn::app::runner::run;

fn main() -> anyhow::Result<()> {
    // 已实现：PipelineManager 见 src/app/pipeline_manager.rs（重构 pipeline1 的创建逻辑）。
    run()?;
    // 实现完成后运行 `cargo run --bin build_pipeline_manager`，check() 会打印自查要点。
    check();
    Ok(())
}

/// 自检：打印验证要点（缓存复用 + 释放重建）。
fn check() {
    println!("自查清单（build_pipeline_manager）：");
    println!("  - 相同 key 第二次 get_or_create 返回缓存的同一个 pipeline");
    println!("  - release 后再次 get_or_create 能重新创建");
    println!("  - renderer 通过 manager 取 pipeline，不再直接持有创建散代码");
}
