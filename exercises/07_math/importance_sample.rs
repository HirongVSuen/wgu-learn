//! 练习 importance_sample：GGX 重要性采样（阶段 7 方向 A / C）
//!
//! 阶段：7A（PBR）/ 7C（光追）
//! 前置：brdf_ggx
//! 依赖：glam
//!
//! 步骤
//! 1. 理解蒙特卡洛积分与 PDF。
//! 2. 对 GGX 半程向量做重要性采样（phi / theta）。
//! 3. 实现采样函数。
//! 4. 验证样本集中在粗糙度对应的方向。
//!
//! 资料
//! - JCGT 2018（GGX 可见法线采样）: https://jcgt.org/published/0007/04/01/
//! - Scratchapixel（蒙特卡洛）: https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/monte-carlo-methods-in-practice/
//!
//! 验收
//! - 蒙特卡洛积分、概率密度、重要性采样
//!
//! 总结
//! 理解重要性采样“按 PDF 采样以降低方差”的核心思想。

use glam::{Vec2, Vec3};

const PI: f32 = std::f32::consts::PI;

/// 采样 GGX 半程向量 h。
fn sample_ggx_half_vector(xi: Vec2, roughness: f32) -> Vec3 {
    let alpha = roughness * roughness;
    // TODO: phi = 2π * xi.x；theta = atan(alpha * sqrt(xi.y / (1 - xi.y)))。
    let _ = (xi, alpha, PI);
    todo!("GGX 重要性采样")
}

/// 自检：验证采样结果是单位向量。
fn check() {
    let h = sample_ggx_half_vector(Vec2::new(0.5, 0.3), 0.4);
    let ok = (h.length() - 1.0).abs() < 1e-3;

    if ok {
        println!("✅ 通过：半程向量是单位长度。");
    } else {
        println!("❌ 未通过：|h| = {:.4}，应为 1。", h.length());
        println!("   提示：h = (cosφ·sinθ, sinφ·sinθ, cosθ)。");
    }
}

fn main() {
    check();
}
