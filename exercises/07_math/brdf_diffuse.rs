//! 练习 brdf_diffuse：Lambert 漫反射 BRDF（阶段 7 方向 A）
//!
//! 阶段：7A（PBR / 光照）
//! 前置：无（只需点积基础）
//! 依赖：glam
//!
//! 学习思路
//! 先理解点积光照 n·l，再理解 BRDF 与能量守恒；
//! Lambert 漫反射 = albedo / π，半球积分后正好等于 albedo。
//!
//! 资料
//! - Filament PBR 指南: https://google.github.io/filament/Filament.html
//! - Physically Based Rendering: https://www.pbr-book.org/
//!
//! 验收
//! - 点积光照、BRDF、能量守恒

use glam::Vec3;

const PI: f32 = std::f32::consts::PI;

/// 返回漫反射 BRDF 值（单位 sr^-1）。
fn brdf_diffuse(albedo: Vec3) -> Vec3 {
    // TODO: 实现 albedo / PI。
    todo!("albedo / PI")
}

fn main() {
    let albedo = Vec3::new(0.8, 0.2, 0.2);
    let _f = brdf_diffuse(albedo);
}
