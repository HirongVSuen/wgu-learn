//! 练习 view_matrix：view / projection 矩阵（数学 M0）
//!
//! 阶段：3（Buffer + MVP 相机）
//! 前置：02_triangle（先有“坐标空间”的直觉）
//! 依赖：glam
//!
//! 学习思路
//! view 把世界空间转到相机空间，projection 转到裁剪空间。用 glam 的
//! `camera` 模块实现，不手推矩阵；重点是会用、能解释。
//!
//! 资料
//! - glam 文档: https://docs.rs/glam/
//! - Scratchapixel（投影矩阵）: https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/
//! - learn-wgpu（uniforms/相机）: https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/
//!
//! 验收
//! - 用 glam 实现 look_at 与 perspective
//! - 不手推矩阵

use glam::{Mat4, Vec3};

/// 相机看向目标点的 view 矩阵（等价于 `glam::camera::rh::view::look_at_mat4`）。
fn view_matrix(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    // TODO: 用 glam 实现（可直接调用 glam::camera::rh::view::look_at_mat4）。
    todo!("look_at(eye, target, up)")
}

/// 透视投影矩阵（等价于 `glam::camera::rh::proj::opengl::perspective`；
/// wgpu 常用 `directx` 变体，深度范围 [0,1]）。
fn projection_matrix(fovy_radians: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    // TODO: 用 glam 实现透视投影。
    todo!("perspective(fovy, aspect, near, far)")
}

fn main() {
    let _v = view_matrix(Vec3::new(0.0, 2.0, 5.0), Vec3::ZERO, Vec3::Y);
    let _p = projection_matrix(45f32.to_radians(), 16.0 / 9.0, 0.1, 100.0);
}
