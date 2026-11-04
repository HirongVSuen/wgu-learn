//! 练习 ecs_world：ECS World + 实体材质绑定（重构）
//!
//! 阶段：5（小项目）
//! 前置：project1
//! 依赖：wgpu + glam
//!
//! 目标（学「造」）
//! 把 project1 里「物体数据 + 相机 + 渲染」的耦合，重构为 ECS 风格的 World：
//! 实体持有组件（变换 / 材质 / 网格），World 每帧收集渲染任务喂给渲染层。
//! 对应引擎化路线「方案三」与「阶段 D」。
//!
//! 步骤
//! 1. 定义 Entity（轻量 ID）与若干组件：Transform、Material、Mesh。
//! 2. 实现 World：按组件类型存储（如 HashMap<Component, HashMap<Entity, T>>）。
//! 3. 提供 spawn / 增删组件 / 查询。
//! 4. 每帧遍历带 Mesh + Material 的实体，生成渲染任务。
//! 5. 重构 project1：物体不再散在 renderer，改由 World 统一管理。
//!
//! 资料
//! - Bevy（Rust ECS）: https://bevyengine.org/learn/
//! - ECS 概念: https://en.wikipedia.org/wiki/Entity_component_system
//!
//! 验收
//! - World 能 spawn 实体、增删组件、按组件查询
//! - 每帧从 World 收集渲染任务
//! - 实体与材质（handle）正确绑定
//!
//! 总结
//! 理解 ECS 的「数据与逻辑分离」：World 只管理数据，渲染任务由查询产生，
//! 是引擎场景/实体管理（阶段 D）的核心范式。
//!
//! 注意：相机可作为一种特殊组件/资源，与实体体系解耦或作为单例资源。

use wgpu_learn::app::runner::run;

fn main() -> anyhow::Result<()> {
    // 已实现：ECS World 见 src/app/（重构 project1 的场景管理）。
    run()?;
    // 实现完成后运行 `cargo run --bin build_ecs_world`，check() 会打印自查要点。
    check();
    Ok(())
}

/// 自检：打印验证要点（实体组件 + 渲染任务收集）。
fn check() {
    println!("自查清单（build_ecs_world）：");
    println!("  - World 能 spawn 实体、增删组件、按组件查询");
    println!("  - 每帧从 World 收集渲染任务");
    println!("  - 实体与材质（handle）正确绑定");
}
