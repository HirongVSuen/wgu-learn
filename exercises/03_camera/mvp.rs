//! 练习 mvp：MVP 组合（数学 M0）
//!
//! 阶段：3（Buffer + MVP 相机）
//! 前置：view_matrix
//! 依赖：glam
//!
//! 学习思路
//! 列向量右乘，顺序是 `projection * view * model`；
//! 每个矩阵作用：model 摆物体、view 摆相机、projection 到裁剪空间。
//!
//! 资料
//! - LearnOpenGL（坐标系统）: https://learnopengl.com/Getting-started/Coordinate-Systems
//! - learn-wgpu（uniforms）: https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/
//!
//! 验收
//! - 用 glam 组合 model / view / projection
//! - 能口头解释每个矩阵的作用

use glam::{Mat4, Vec3};

/// 组合 MVP：`projection * view * model`。
fn compute_mvp(model: Mat4, view: Mat4, projection: Mat4) -> Mat4 {
    // TODO: 按正确顺序相乘。
    todo!("projection * view * model")
}

fn main() {
    // 示例：单位模型，相机在 +Z 看向原点。
    // 注意：glam 0.34 的相机函数在 `glam::camera` 模块下。
    let model = Mat4::IDENTITY;
    let view = glam::camera::rh::view::look_at_mat4(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
    let projection =
        glam::camera::rh::proj::opengl::perspective(45f32.to_radians(), 16.0 / 9.0, 0.1, 100.0);
    let _mvp = compute_mvp(model, view, projection);
}
