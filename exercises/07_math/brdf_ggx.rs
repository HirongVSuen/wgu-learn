//! 练习 brdf_ggx：GGX 微表面法线分布 D（阶段 7 方向 A）
//!
//! 对应阶段 7A：
//! - 微表面、Cook-Torrance
//!
//! GGX / Trowbridge-Reitz 法线分布函数：
//! `D(h) = α² / (π * ((n·h)² * (α² - 1) + 1)²)`，其中 α = roughness²。

use glam::Vec3;

const PI: f32 = std::f32::consts::PI;

/// GGX 法线分布函数 D(h)。
fn ggx_d(normal: Vec3, half_vec: Vec3, roughness: f32) -> f32 {
    let alpha = roughness * roughness;
    let ndoth = normal.dot(half_vec).max(0.0);
    // TODO: 代入 GGX 公式。
    let _ = (alpha, ndoth, PI);
    todo!("GGX D")
}

fn main() {
    let n = Vec3::Y;
    let h = Vec3::new(0.1, 0.9, 0.0).normalize();
    let _d = ggx_d(n, h, 0.5);
}
