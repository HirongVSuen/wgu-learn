//! 练习 mvp：MVP 组合（数学 M0）
//!
//! 阶段：3（Buffer + MVP 相机）
//! 前置：view_matrix
//! 依赖：glam
//!
//! 步骤
//! 1. 定义 model（旋转/缩放）。
//! 2. 求 view、projection。
//! 3. 按 `projection * view * model` 顺序相乘。
//! 4. 用 MVP 变换一个顶点，观察它落到裁剪空间。
//!
//! 资料
//! - LearnOpenGL（坐标系统）: https://learnopengl.com/Getting-started/Coordinate-Systems
//! - learn-wgpu（uniforms）: https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/
//!
//! 验收
//! - 用 glam 组合 model / view / projection
//! - 能口头解释每个矩阵的作用
//!
//! 总结
//! 牢记列向量右乘顺序 P*V*M，能说出每个矩阵把点从哪个空间转到哪个空间。

use glam::{Mat4, Vec3};

/// 组合 MVP：`projection * view * model`。
fn compute_mvp(model: Mat4, view: Mat4, projection: Mat4) -> Mat4 {
    // TODO: 按正确顺序相乘。
    todo!("projection * view * model")
}

/// 自检：对比标准结果，给出通过/未通过反馈。
fn check() {
    let model = Mat4::from_rotation_y(0.5);
    let view = glam::camera::rh::view::look_at_mat4(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
    let projection =
        glam::camera::rh::proj::opengl::perspective(45f32.to_radians(), 16.0 / 9.0, 0.1, 100.0);

    let got = compute_mvp(model, view, projection);
    let expected = projection * view * model;
    let ok = got
        .to_cols_array()
        .iter()
        .zip(expected.to_cols_array())
        .all(|(a, b)| (a - b).abs() < 1e-3);

    if ok {
        println!("✅ 通过：MVP = projection * view * model。");
    } else {
        println!("❌ 未通过。提示：顺序是 projection * view * model（列向量右乘）。");
    }
}

fn main() {
    check();
}
