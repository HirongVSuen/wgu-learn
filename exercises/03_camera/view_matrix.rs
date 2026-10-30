//! 练习 view_matrix：view / projection 矩阵（数学 M0）
//!
//! 对应阶段 3 + 数学 M0：
//! - 用 glam 实现相机 view 矩阵（look_at）
//! - 用 glam 实现透视投影（perspective）
//! - 不用手推矩阵，直接调用 glam。

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
