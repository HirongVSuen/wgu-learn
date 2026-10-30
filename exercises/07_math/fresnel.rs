//! 练习 fresnel：Schlick Fresnel（阶段 7 方向 A）
//!
//! 对应阶段 7A：
//! - 微表面、Cook-Torrance
//!
//! Schlick Fresnel 近似：`F = F0 + (1 - F0) * (1 - cosθ)^5`。

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
