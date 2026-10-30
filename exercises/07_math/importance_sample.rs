//! 练习 importance_sample：GGX 重要性采样（阶段 7 方向 A）
//!
//! 对应阶段 7A / 方向 C：
//! - 蒙特卡洛积分、概率密度、重要性采样
//!
//! 用两个 [0,1) 随机数对 GGX 半程向量做重要性采样。
//! 参考：Sampling the GGX Distribution of Visible Normals (JCGT 2018)。

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
