//! 练习 fresnel：Schlick Fresnel（阶段 7 方向 A）
//!
//! 阶段：7A（PBR / 光照）
//! 前置：brdf_diffuse
//! 依赖：glam
//!
//! 步骤
//! 1. 理解掠射角反射增强现象。
//! 2. 确定 F0（绝缘体 0.04 / 金属用 albedo）。
//! 3. 实现 Schlick 近似。
//! 4. 观察 cosθ→0 时 F 趋于 1。
//!
//! 资料
//! - Filament（Fresnel 章节）: https://google.github.io/filament/Filament.html
//! - Schlick 近似参考: https://en.wikipedia.org/wiki/Schlick%27s_approximation
//!
//! 验收
//! - 实现并解释 Schlick Fresnel
//!
//! 总结
//! 理解 Fresnel 效应与 Schlick 近似，掌握 F0 与材质的关系。

use glam::Vec3;

/// Schlick Fresnel 近似。
fn fresnel_schlick(cos_theta: f32, f0: Vec3) -> Vec3 {
    // TODO: 实现 F0 + (1 - F0) * (1 - cosθ)^5。
    todo!("Schlick Fresnel")
}

fn main() {
    // 绝缘体基础反射率约 0.04。
    let f0 = Vec3::splat(0.04);
    let _f = fresnel_schlick(0.6, f0);
}
