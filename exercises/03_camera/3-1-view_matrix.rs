//! 练习 view_matrix：view / projection 矩阵（数学 M0）
//!
//! 阶段：3（Buffer + MVP 相机）
//! 前置：02_triangle（先有“坐标空间”的直觉）
//! 依赖：glam
//!
//! 步骤
//! 1. 用 `glam::camera::rh::view::look_at_mat4(eye, target, up)` 求 view。
//! 2. 用 `glam::camera::rh::proj::opengl::perspective(fov, aspect, near, far)` 求 projection。
//! 3. 确认 wgpu 用 directx 变体（深度 [0,1]）。
//! 4. 打印矩阵或用已知点验证变换结果。
//!
//! 资料
//! - glam 文档: https://docs.rs/glam/
//! - Scratchapixel（投影矩阵）: https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/
//! - learn-wgpu（uniforms/相机）: https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/
//!
//! 验收
//! - 用 glam 实现 look_at 与 perspective
//! - 不手推矩阵
//!
//! 总结
//! 会用 glam 求 view/projection，理解两者的作用与 wgpu 的 NDC 约定。

use glam::{Mat4, Vec3};

/// 相机看向目标点的 view 矩阵（等价于 `glam::camera::rh::view::look_at_mat4`）。
fn view_matrix(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    // TODO: 用 glam 实现（可直接调用 glam::camera::rh::view::look_at_mat4）。
    todo!("look_at(eye, target, up)")
}

/// 透视投影矩阵。本练习用 `opengl` 变体做数学对照（深度 [-1,1]）；
/// 实际 wgpu 的深度约定是 `directx` 变体（深度 [0,1]），需用
/// `glam::camera::rh::proj::directx::perspective_*` 才能在 wgpu 中正确使用。
fn projection_matrix(fovy_radians: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    // TODO: 用 glam 实现透视投影。
    todo!("perspective(fovy, aspect, near, far)")
}

/// 自检：对比 glam 标准结果，给出通过/未通过反馈。
fn check() {
    let eye = Vec3::new(0.0, 0.0, 3.0);
    let target = Vec3::ZERO;
    let up = Vec3::Y;

    let v = view_matrix(eye, target, up);
    let v_ok = v
        .to_cols_array()
        .iter()
        .zip(glam::camera::rh::view::look_at_mat4(eye, target, up).to_cols_array())
        .all(|(a, b)| (a - b).abs() < 1e-3);

    let (fovy, aspect, near, far) = (45f32.to_radians(), 16.0 / 9.0, 0.1, 100.0);
    let p = projection_matrix(fovy, aspect, near, far);
    let p_ok = p
        .to_cols_array()
        .iter()
        .zip(glam::camera::rh::proj::opengl::perspective(fovy, aspect, near, far).to_cols_array())
        .all(|(a, b)| (a - b).abs() < 1e-3);

    if v_ok && p_ok {
        println!("✅ 通过：view 与 projection 矩阵计算正确。");
    } else {
        println!("❌ 未通过。提示：view 用 glam::camera::rh::view::look_at_mat4，");
        println!("   projection 用 glam::camera::rh::proj::opengl::perspective。");
    }
}

fn main() {
    check();
}
