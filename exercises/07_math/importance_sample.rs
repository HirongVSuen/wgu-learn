//! 练习 importance_sample：GGX 重要性采样（阶段 7 方向 A / C）
//!
//! 阶段：7A（PBR）/ 7C（光追）
//! 前置：brdf_ggx
//! 依赖：glam
//!
//! 学习思路
//! 蒙特卡洛积分需要按 PDF 采样；对 GGX 半程向量做重要性采样，
//! 让样本集中在贡献大的方向，降低方差。
//!
//! 资料
//! - JCGT 2018（GGX 可见法线采样）: https://jcgt.org/published/0007/04/01/
//! - Scratchapixel（蒙特卡洛）: https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/monte-carlo-methods-in-practice/
//!
//! 验收
//! - 蒙特卡洛积分、概率密度、重要性采样

use glam::{Vec2, Vec3};

const PI: f32 = std::f32::consts::PI;

/// 采样 GGX 半程向量 h。
fn sample_ggx_half_vector(xi: Vec2, roughness: f32) -> Vec3 {
    let alpha = roughness * roughness;
    // TODO: phi = 2π * xi.x；theta = atan(alpha * sqrt(xi.y / (1 - xi.y)))。
    let _ = (xi, alpha, PI);
    todo!("GGX 重要性采样")
}

fn main() {
    let xi = Vec2::new(0.5, 0.3);
    let _h = sample_ggx_half_vector(xi, 0.4);
}
