//! 练习 render_graph：Render Graph + 多 Pass（重构）
//!
//! 阶段：6（按需扩展）
//! 前置：postprocess1 / shadow1
//! 依赖：wgpu
//!
//! 目标（学「造」）
//! 把 postprocess / shadow 里手写的「先这个 pass 再那个 pass」，重构为声明式
//! Render Graph：node（pass）+ 资源依赖，自动推导执行顺序与资源复用。
//! 对应引擎化路线「方案二」与「阶段 E」。
//!
//! 步骤
//! 1. 定义 Pass 节点：输入/输出渲染目标（attachment）、执行闭包。
//! 2. 定义 Render Graph：节点集合 + 资源依赖边。
//! 3. 拓扑排序推导执行顺序。
//! 4. 渲染目标（离屏纹理）复用与生命周期管理。
//! 5. 重构 postprocess1/shadow1 的 pass 为 graph 节点。
//!
//! 资料
//! - wgpu RenderPass: https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html
//! - frame graph 思路: https://www.gamedev.net/tutorials/programming/graphics/frame-graphs-r3535/
//!
//! 验收
//! - pass 以声明式 node 表示，支持资源依赖
//! - 拓扑排序自动推导执行顺序
//! - 离屏渲染目标可复用，无多余开销
//!
//! 总结
//! 理解 frame graph：把「硬编码的多 pass 流程」变为「配置化」，自动调度与资源复用，
//! 是现代引擎渲染器（阶段 E）的分水岭。
//!
//! 注意：LoadOp / StoreOp 在多 pass 间的数据传递是资源复用与性能的关键。

use wgpu_learn::app::runner::run;

fn main() -> anyhow::Result<()> {
    // 已实现：Render Graph 见 src/app/（重构 postprocess/shadow 的多 pass 调度）。
    run()?;
    // 实现完成后运行 `cargo run --bin build_render_graph`，check() 会打印自查要点。
    check();
    Ok(())
}

/// 自检：打印验证要点（声明式 pass + 自动调度）。
fn check() {
    println!("自查清单（build_render_graph）：");
    println!("  - pass 以声明式 node 表示，支持资源依赖");
    println!("  - 拓扑排序自动推导执行顺序");
    println!("  - 离屏渲染目标可复用，无多余开销");
}
