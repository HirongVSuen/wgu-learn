//! 练习 fresnel：Schlick Fresnel（阶段 7 方向 A）
//!
//! 阶段：7A（PBR / 光照）
//! 前置：brdf_diffuse
//! 依赖：glam
//!
//! 学习思路
//! Fresnel 描述掠射角反射增强；Schlick 近似 `F = F0 + (1-F0)(1-cosθ)^5`。
//! F0 是法线入射的基础反射率（绝缘体约 0.04，金属用 albedo 作为 F0）。
//!
//! 资料
//! - Filament（Fresnel 章节）: https://google.github.io/filament/Filament.html
//! - Schlick 近似参考: https://en.wikipedia.org/wiki/Schlick%27s_approximation
//!
//! 验收
//! - 实现并解释 Schlick Fresnel

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
