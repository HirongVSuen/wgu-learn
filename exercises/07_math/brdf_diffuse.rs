//! 练习 brdf_diffuse：Lambert 漫反射 BRDF（阶段 7 方向 A）
//!
//! 对应阶段 7A：
//! - 点积光照、BRDF、能量守恒
//!
//! Lambert 漫反射 BRDF = albedo / π（能量守恒，半球积分后等于 albedo）。

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
