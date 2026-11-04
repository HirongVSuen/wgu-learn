//! 练习 asset_store：资源管理器 Assets + Handle（重构）
//!
//! 阶段：4（纹理 + 深度）
//! 前置：texture1
//! 依赖：wgpu
//!
//! 目标（学「造」）
//! 把 texture1 里纹理的创建/上传，重构为通用资源管理器：`Assets<T>` 仓库 +
//! 轻量 `Handle<T>`（ID）引用，支持缓存、查找、释放。对应引擎化路线「阶段 C」。
//!
//! 步骤
//! 1. 定义泛型 `Assets<T>`：内部 `HashMap<Handle<T>, T>` 仓库。
//! 2. 定义 `Handle<T>`：轻量 ID（如 `u64` 新类型）。
//! 3. 实现 `insert(T) -> Handle<T>`、`get(&Handle<T>) -> Option<&T>`、`remove`。
//! 4. 把纹理接入：`insert(texture)` 得到 handle，采样时 `get` 取出。
//! 5. 重构 renderer：不再直接持有 texture，改持 handle。
//!
//! 资料
//! - wgpu Texture: https://docs.rs/wgpu/latest/wgpu/struct.Texture.html
//! - 句柄/资源池模式: https://en.wikipedia.org/wiki/Reference_counting
//!
//! 验收
//! - Assets<T> 能按 handle 存取、查找、释放
//! - 纹理通过 handle 引用，renderer 不直接持有纹理
//! - 支持异步/热重载的扩展点（handle 解耦）
//!
//! 总结
//! 理解「资产仓库 + 句柄」模式：用轻量 ID 解耦资源的生产与消费，
//! 是异步加载、热重载、生命周期管理的基础。
//!
//! 注意：纹理的 sRGB 格式（`Rgba8UnormSrgb`）与上传（write_texture）应一并纳入管理。

use wgpu_learn::app::runner::run;

fn main() -> anyhow::Result<()> {
    // 已实现：Assets<T> + Handle<T> 见 src/app/（重构 texture1 的纹理管理）。
    run()?;
    // 实现完成后运行 `cargo run --bin build_asset_store`，check() 会打印自查要点。
    check();
    Ok(())
}

/// 自检：打印验证要点（资产仓库 + 句柄解耦）。
fn check() {
    println!("自查清单（build_asset_store）：");
    println!("  - Assets<T> 能按 handle 存取、查找、释放");
    println!("  - 纹理通过 handle 引用，renderer 不直接持有纹理");
    println!("  - handle 解耦后，可扩展异步加载 / 热重载");
}
