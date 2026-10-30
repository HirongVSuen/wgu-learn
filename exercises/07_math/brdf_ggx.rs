//! 练习 brdf_ggx：GGX 微表面法线分布 D（阶段 7 方向 A）
//!
//! 阶段：7A（PBR / 光照）
//! 前置：brdf_diffuse
//! 依赖：glam
//!
//! 步骤
//! 1. 理解微表面法线分布 D 的含义。
//! 2. 令 α = roughness²。
//! 3. 实现 GGX 公式。
//! 4. 改变 roughness，观察 D 的集中/分散。
//!
//! 资料
//! - Filament（微表面章节）: https://google.github.io/filament/Filament.html
//! - JCGT 2018（GGX 采样）: https://jcgt.org/published/0007/04/01/
//!
//! 验收
//! - 微表面、Cook-Torrance 的 D 项
//!
//! 总结
//! 理解微表面理论与 GGX 分布对高光形状的影响。
//!
//! 公式：`D(h) = α² / (π * ((n·h)² * (α² - 1) + 1)²)`，α = roughness²。

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

/// 自检：验证 n == h 时 D = 1/(π · roughness⁴)。
fn check() {
    let n = Vec3::Y;
    let h = Vec3::Y; // n·h = 1
    let roughness = 0.5;
    let got = ggx_d(n, h, roughness);
    let expected = 1.0 / (PI * roughness.powi(4));
    let ok = (got - expected).abs() < 1e-3;

    if ok {
        println!("✅ 通过：GGX D(n,n) = 1/(π·roughness⁴) 正确。");
    } else {
        println!("❌ 未通过：得到 {got:.4}，期望 {expected:.4}。");
        println!("   提示：α=roughness²，n·h=1 时 D = 1/(π·α²)。");
    }
}

fn main() {
    check();
}
