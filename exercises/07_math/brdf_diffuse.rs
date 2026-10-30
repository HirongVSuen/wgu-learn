//! 练习 brdf_diffuse：Lambert 漫反射 BRDF（阶段 7 方向 A）
//!
//! 阶段：7A（PBR / 光照）
//! 前置：无（只需点积基础）
//! 依赖：glam
//!
//! 步骤
//! 1. 回忆点积光照 n·l。
//! 2. 理解 BRDF 是“出射 / 入射辐射度之比”。
//! 3. 实现 albedo / π。
//! 4. 验证半球积分 ≈ albedo（能量守恒）。
//!
//! 资料
//! - Filament PBR 指南: https://google.github.io/filament/Filament.html
//! - Physically Based Rendering: https://www.pbr-book.org/
//!
//! 验收
//! - 点积光照、BRDF、能量守恒
//!
//! 总结
//! 理解漫反射 BRDF 与能量守恒：除以 π 使半球积分等于 albedo。

use glam::Vec3;

const PI: f32 = std::f32::consts::PI;

/// 返回漫反射 BRDF 值（单位 sr^-1）。
fn brdf_diffuse(albedo: Vec3) -> Vec3 {
    // TODO: 实现 albedo / PI。
    todo!("albedo / PI")
}

/// 自检：验证结果是否为 albedo / π。
fn check() {
    let albedo = Vec3::new(0.8, 0.2, 0.2);
    let got = brdf_diffuse(albedo);
    let expected = albedo / PI;
    let ok = (got - expected).length() < 1e-4;

    if ok {
        println!("✅ 通过：brdf_diffuse = albedo / π。");
    } else {
        println!("❌ 未通过：得到 {got:?}，期望 {expected:?}。");
        println!("   提示：Lambert = albedo / π。");
    }
}

fn main() {
    check();
}
